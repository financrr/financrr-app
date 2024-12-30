use std::collections::HashMap;
use std::ops::Deref;
use std::sync::{Arc, LazyLock, OnceLock};

use deschuler::scheduler::job::Job;
use deschuler::scheduler::tokio_scheduler::config::TokioSchedulerConfig;
use deschuler::scheduler::tokio_scheduler::TokioScheduler;
use deschuler::scheduler::Scheduler;
use sea_orm::{EntityName, EntityTrait, Set};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use tokio::sync::RwLock;
use tracing::error;
use utoipa::ToSchema;

use db_iterator::process_entity;
use entity::recurring_transaction;
use entity::recurring_transaction::Model;
use entity::utility::time::get_now;
use utility::datetime::{convert_chrono_to_time, convert_time_to_chrono};
use utility::snowflake::entity::Snowflake;

use crate::api::error::api::ApiError;
use crate::api::pagination::PageSizeParam;
use crate::database::entity::{count, delete, find_all_paginated, find_one_or_error, insert, update};
use crate::util::cron::get_cron_builder_config_default;
use crate::wrapper::entity::account::Account;
use crate::wrapper::entity::transaction::dto::TransactionDTO;
use crate::wrapper::entity::transaction::recurring::dto::RecurringTransactionDTO;
use crate::wrapper::entity::transaction::recurring::recurring_rule::RecurringRule;
use crate::wrapper::entity::transaction::template::TransactionTemplate;
use crate::wrapper::entity::transaction::Transaction;
use crate::wrapper::entity::{TableName, WrapperEntity};
use crate::wrapper::processor::db_iterator;
use crate::wrapper::processor::db_iterator::{CountAllFn, FindAllPaginatedFn, JobFn};
use crate::wrapper::types::phantom::{Identifiable, Phantom};
use crate::{permission_impl, SNOWFLAKE_GENERATOR};

pub(crate) mod dto;
pub(crate) mod recurring_rule;
pub(crate) mod validation;

type JobMap = LazyLock<Arc<RwLock<HashMap<Snowflake, Arc<Job>>>>>;

static SCHEDULER: OnceLock<Arc<RwLock<TokioScheduler>>> = OnceLock::new();
static JOBS: JobMap = LazyLock::new(|| Arc::new(RwLock::new(HashMap::new())));

const CHANNEL_SIZE: usize = 10240;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub(crate) struct RecurringTransaction {
    #[serde(rename = "id")]
    pub(crate) snowflake: Snowflake,
    pub(crate) template_id: Phantom<TransactionTemplate>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub(crate) last_executed_at: Option<OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub(crate) next_executed_at: Option<OffsetDateTime>,
    pub(crate) recurring_rule: RecurringRule,
    #[serde(with = "time::serde::rfc3339")]
    pub(crate) created_at: OffsetDateTime,
}

impl RecurringTransaction {
    pub(crate) async fn init() {
        Self::redo_missed_transactions().await;
        Self::schedule_recurring_transactions().await;
    }

    async fn redo_missed_transactions() {
        let (count_all, find_all_paginated) = Self::get_count_all_and_find_all_fns();
        let job: JobFn<Self> = Arc::new(move |transaction: Self| {
            let transaction = transaction.clone();
            Box::pin(async move { transaction.redo_missed_transactions_job().await })
        });

        process_entity(count_all, find_all_paginated, job).await;
    }

    async fn schedule_recurring_transactions() {
        let (count_all, find_all_paginated) = Self::get_count_all_and_find_all_fns();
        let job: JobFn<Self> = Arc::new(move |transaction: Self| {
            let transaction = transaction.clone();
            Box::pin(async move { transaction.start_recurring_transaction().await })
        });

        process_entity(count_all, find_all_paginated, job).await;
    }

    fn get_count_all_and_find_all_fns() -> (CountAllFn, FindAllPaginatedFn<Self>) {
        let count_all: CountAllFn = Arc::new(|| Box::pin(Self::count_all()));
        let find_all_paginated: FindAllPaginatedFn<Self> = Arc::new(move |page_size: PageSizeParam| {
            let page_size = page_size.clone();
            Box::pin(Self::find_all_paginated(page_size))
        });

        (count_all, find_all_paginated)
    }

    async fn redo_missed_transactions_job(&self) -> Result<(), ApiError> {
        if let Some(last_executed) = self.last_executed_at {
            let now_chrono = convert_time_to_chrono(&get_now())?;
            let cron = self.recurring_rule.to_cron()?;
            let last_executed = convert_time_to_chrono(&last_executed)?;
            let mut next_occurrence = cron.find_next_occurrence(&last_executed, false)?;

            while next_occurrence < now_chrono {
                let next_occurrence_time = convert_chrono_to_time(&next_occurrence)?;
                self.handle_job(next_occurrence_time).await;

                next_occurrence = self.recurring_rule.to_cron()?.find_next_occurrence(&next_occurrence, false)?;
            }
        }

        Ok(())
    }

    pub(crate) async fn new(dto: RecurringTransactionDTO) -> Result<Self, ApiError> {
        let recurring_rule = RecurringRule::from(dto.recurring_rule);
        recurring_rule.to_cron()?; // doing this to check if the cron is valid

        let snowflake = SNOWFLAKE_GENERATOR.next_id()?;
        let active_model = recurring_transaction::ActiveModel {
            id: Set(snowflake),
            template: Set(dto.template_id.get_id().id),
            recurring_rule: Set(recurring_rule.to_json_value()?),
            last_executed_at: Set(None),
            created_at: Set(get_now()),
        };

        let model = insert(active_model).await?;
        let transaction = Self::from(model);

        let template = dto.template_id.fetch_inner().await?;
        //grant permission
        if let Some(source) = template.source_id.as_ref() {
            Account::assign_permissions_from_account(&transaction, source.get_id()).await?;
        }
        if let Some(destination) = template.destination_id.as_ref() {
            Account::assign_permissions_from_account(&transaction, destination.get_id()).await?;
        }

        //starting the recurring transaction
        transaction.start_recurring_transaction().await?;

        Ok(transaction)
    }

    pub(crate) async fn delete(self) -> Result<(), ApiError> {
        delete(recurring_transaction::Entity::delete_by_id(self.snowflake)).await?;

        let map = get_jobs();
        let map = map.write().await;
        if let Some(job) = map.get(&self.snowflake) {
            job.interrupt();
        }

        Ok(())
    }

    pub(crate) async fn update(&self, dto: RecurringTransactionDTO) -> Result<Self, ApiError> {
        let recurring_rule = RecurringRule::from(dto.recurring_rule);
        recurring_rule.to_cron()?; // doing this to check if the cron is valid
        let active_model = recurring_transaction::ActiveModel {
            id: Set(self.snowflake.id),
            template: Set(dto.template_id.get_id().id),
            recurring_rule: Set(recurring_rule.to_json_value()?),
            last_executed_at: Set(self.last_executed_at),
            created_at: Set(self.created_at),
        };
        let model = update(active_model).await?;
        let transaction = Self::from(model);

        self.stop_recurring_transaction().await?;
        transaction.start_recurring_transaction().await?;

        Ok(transaction)
    }

    async fn start_recurring_transaction(&self) -> Result<(), ApiError> {
        let binding = get_recurring_transaction_scheduler();
        let cron = self.recurring_rule.to_cron()?;

        let transaction = self.clone();
        let job = Job::new_async(Box::new(move |now| {
            let transaction = transaction.clone();
            Box::pin(async move {
                let now = convert_chrono_to_time(&now).expect("Could not convert chrono to time!");
                transaction.handle_job(now).await;
            })
        }));

        let mut scheduler = binding.write().await;
        let job = Arc::new(job);
        scheduler.schedule_job(cron, job.clone());

        let binding = get_jobs();
        binding.write().await.insert(self.snowflake, job);

        Ok(())
    }

    async fn stop_recurring_transaction(&self) -> Result<(), ApiError> {
        let binding = get_jobs();
        let mut jobs = binding.write().await;
        if let Some(job) = jobs.remove(&self.snowflake) {
            job.interrupt();
        }

        Ok(())
    }

    async fn handle_job(&self, now: OffsetDateTime) {
        if let Err(err) = self.recurring_transaction_job_task(now).await {
            error!("Could not execute recurring transaction job. Error: {:?}", err);
        } else if let Err(err) = self.update_last_run(now).await {
            error!("Could not update last run. Error: {:?}", err);
        }
    }

    async fn recurring_transaction_job_task(&self, now: OffsetDateTime) -> Result<(), ApiError> {
        let template = Arc::new(self.template_id.fetch_inner().await?);
        let dto = TransactionDTO::from_template(template, now).await?;
        Transaction::new(dto).await?;

        Ok(())
    }

    async fn update_last_run(&self, last_run: OffsetDateTime) -> Result<(), ApiError> {
        let mut active_model = self.to_active_model();
        active_model.last_executed_at = Set(Some(last_run));
        update(active_model).await?;

        Ok(())
    }

    fn to_active_model(&self) -> recurring_transaction::ActiveModel {
        recurring_transaction::ActiveModel {
            id: Set(self.snowflake.id),
            template: Set(self.template_id.get_id().id),
            recurring_rule: Set(self.recurring_rule.to_json_value().expect("Could not parse recurring rule to json!")),
            last_executed_at: Set(self.last_executed_at),
            created_at: Set(self.created_at),
        }
    }

    pub(crate) async fn count_all_by_user_id(user_id: Snowflake) -> Result<u64, ApiError> {
        count(recurring_transaction::Entity::find_all_by_user_id(user_id)).await
    }

    pub(crate) async fn find_all_by_user_id_paginated(
        user_id: Snowflake,
        page_size: &PageSizeParam,
    ) -> Result<Vec<Self>, ApiError> {
        Ok(find_all_paginated(recurring_transaction::Entity::find_all_by_user_id(user_id), page_size)
            .await?
            .into_iter()
            .map(Self::from)
            .collect())
    }

    pub(crate) async fn find_by_id(id: Snowflake) -> Result<Self, ApiError> {
        find_one_or_error(recurring_transaction::Entity::find_by_id(id)).await.map(Self::from)
    }

    pub(crate) async fn count_all() -> Result<u64, ApiError> {
        count(recurring_transaction::Entity::find()).await
    }

    pub(crate) async fn find_all_paginated(page_size: PageSizeParam) -> Result<Vec<Self>, ApiError> {
        Ok(find_all_paginated(recurring_transaction::Entity::find(), &page_size)
            .await?
            .into_iter()
            .map(Self::from)
            .collect())
    }
}

permission_impl!(RecurringTransaction);

impl From<Model> for RecurringTransaction {
    fn from(value: Model) -> Self {
        let recurring_rule: RecurringRule =
            RecurringRule::from_json_value(value.recurring_rule).expect("Failed to parse recurring rule");
        Self {
            snowflake: Snowflake::from(value.id),
            template_id: Phantom::from(value.template),
            last_executed_at: value.last_executed_at,
            next_executed_at: recurring_rule.find_next_occurrence(&get_now()),
            recurring_rule,
            created_at: value.created_at,
        }
    }
}

impl Identifiable for RecurringTransaction {
    async fn find_by_id(id: Snowflake) -> Result<Self, ApiError> {
        find_one_or_error(recurring_transaction::Entity::find_by_id(id)).await.map(Self::from)
    }
}

impl TableName for RecurringTransaction {
    fn table_name() -> &'static str {
        recurring_transaction::Entity.table_name()
    }
}

impl WrapperEntity for RecurringTransaction {
    fn get_id(&self) -> Snowflake {
        self.snowflake
    }
}

pub(crate) fn get_recurring_transaction_scheduler() -> Arc<RwLock<TokioScheduler>> {
    SCHEDULER.get_or_init(|| Arc::new(RwLock::new(create_tokio_scheduler()))).clone()
}

pub(crate) fn get_jobs() -> Arc<RwLock<HashMap<Snowflake, Arc<Job>>>> {
    JOBS.deref().clone()
}

fn create_tokio_scheduler() -> TokioScheduler {
    let builder_config = get_cron_builder_config_default().expect("Could not create default BuilderConfig!");
    let config = TokioSchedulerConfig {
        channel_size: CHANNEL_SIZE,
        builder_config,
    };

    let mut scheduler = TokioScheduler::new(config);
    scheduler.start();

    scheduler
}
