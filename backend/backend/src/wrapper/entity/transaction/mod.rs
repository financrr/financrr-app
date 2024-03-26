use sea_orm::ActiveValue::Set;
use sea_orm::{EntityName, EntityTrait};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use tokio::time::Duration;
use utoipa::ToSchema;

use entity::transaction;
use entity::transaction::Model;
use entity::utility::time::get_now;

use crate::api::error::api::ApiError;
use crate::api::pagination::PageSizeParam;
use crate::databases::entity::{count, delete, find_all_paginated, find_one_or_error, insert, update};
use crate::event::lifecycle::transaction::{TransactionCreation, TransactionDeletion, TransactionUpdate};
use crate::event::GenericEvent;
use crate::wrapper::entity::account::Account;
use crate::wrapper::entity::budget::Budget;
use crate::wrapper::entity::currency::Currency;
use crate::wrapper::entity::transaction::dto::TransactionDTO;
use crate::wrapper::entity::{TableName, WrapperEntity};
use crate::wrapper::permission::{
    HasPermissionByIdOrError, HasPermissionOrError, Permission, PermissionByIds, Permissions,
};
use crate::wrapper::types::phantom::{Identifiable, Phantom};

pub(crate) mod dto;
pub(crate) mod search;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub(crate) struct Transaction {
    pub(crate) id: i32,
    pub(crate) source_id: Option<Phantom<Account>>,
    pub(crate) destination_id: Option<Phantom<Account>>,
    pub(crate) amount: i64,
    pub(crate) currency_id: Phantom<Currency>,
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) budget_id: Option<Phantom<Budget>>,
    #[serde(with = "time::serde::rfc3339")]
    pub(crate) created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub(crate) executed_at: OffsetDateTime,
}

impl Transaction {
    pub(crate) async fn new(dto: TransactionDTO, user_id: i32) -> Result<Self, ApiError> {
        let active_model = transaction::ActiveModel {
            id: Default::default(),
            source: Set(dto.source_id.map(|source| source.get_id())),
            destination: Set(dto.destination_id.map(|destination| destination.get_id())),
            amount: Set(dto.amount),
            currency: Set(dto.currency_id.get_id()),
            name: Set(dto.name),
            description: Set(dto.description),
            budget: Set(dto.budget_id.map(|budget| budget.get_id())),
            created_at: Set(get_now()),
            executed_at: Set(dto.executed_at),
        };
        let model = insert(active_model).await?;

        let transaction = Self::from(model);

        //grant permission
        transaction.add_permission(user_id, Permissions::all()).await?;

        // check if execute_at is in the future
        if transaction.executed_at > get_now() {
            let delay = transaction.executed_at - get_now();
            let delay = Duration::new(delay.whole_seconds() as u64, 0);
            TransactionCreation::fire_scheduled(TransactionCreation::new(transaction.clone()), delay);
        } else {
            TransactionCreation::fire(TransactionCreation::new(transaction.clone()));
        }

        Ok(transaction)
    }

    pub(crate) async fn update(self, updated_dto: TransactionDTO) -> Result<Self, ApiError> {
        let active_model = transaction::ActiveModel {
            id: Set(self.id),
            source: Set(updated_dto.source_id.map(|source| source.get_id())),
            destination: Set(updated_dto.destination_id.map(|destination| destination.get_id())),
            amount: Set(updated_dto.amount),
            currency: Set(updated_dto.currency_id.get_id()),
            name: Set(updated_dto.name),
            description: Set(updated_dto.description),
            budget: Set(updated_dto.budget_id.map(|budget| budget.get_id())),
            created_at: Set(self.created_at),
            executed_at: Set(updated_dto.executed_at),
        };
        let transaction = Self::from(update(active_model).await?);

        TransactionUpdate::fire(TransactionUpdate::new(self.clone(), transaction.clone()));

        Ok(transaction)
    }

    pub(crate) async fn delete(self) -> Result<(), ApiError> {
        delete(transaction::Entity::delete_by_id(self.id)).await?;

        TransactionDeletion::fire(TransactionDeletion::new(self.clone()));

        Ok(())
    }

    pub(crate) async fn find_by_id(id: i32) -> Result<Self, ApiError> {
        Ok(Self::from(find_one_or_error(transaction::Entity::find_by_id(id), "Transaction").await?))
    }

    pub(crate) async fn find_all_by_user_paginated(
        user_id: i32,
        page_size: &PageSizeParam,
    ) -> Result<Vec<Self>, ApiError> {
        Ok(find_all_paginated(transaction::Entity::find_all_by_user(user_id), page_size)
            .await?
            .into_iter()
            .map(Self::from)
            .collect())
    }

    pub(crate) async fn count_all_by_user(user_id: i32) -> Result<u64, ApiError> {
        count(transaction::Entity::find_all_by_user(user_id)).await
    }

    pub(crate) async fn find_all_paginated(page_size: PageSizeParam) -> Result<Vec<Self>, ApiError> {
        Ok(find_all_paginated(transaction::Entity::find(), &page_size).await?.into_iter().map(Self::from).collect())
    }

    pub(crate) async fn count_all() -> Result<u64, ApiError> {
        count(transaction::Entity::find()).await
    }
}

impl TableName for Transaction {
    fn table_name() -> &'static str {
        transaction::Entity.table_name()
    }
}

impl WrapperEntity for Transaction {
    fn get_id(&self) -> i32 {
        self.id
    }
}

impl PermissionByIds for Transaction {}

impl Permission for Transaction {}

impl HasPermissionOrError for Transaction {}

impl HasPermissionByIdOrError for Transaction {}

impl Identifiable for Transaction {
    async fn from_id(id: i32) -> Result<Self, ApiError> {
        Self::find_by_id(id).await
    }
}

impl From<transaction::Model> for Transaction {
    fn from(value: Model) -> Self {
        Self {
            id: value.id,
            source_id: Phantom::from_option(value.source),
            destination_id: Phantom::from_option(value.destination),
            amount: value.amount,
            currency_id: Phantom::new(value.currency),
            name: value.name,
            description: value.description,
            budget_id: Phantom::from_option(value.budget),
            created_at: value.created_at,
            executed_at: value.executed_at,
        }
    }
}
