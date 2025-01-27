pub use super::_entities::external_bank_institutions::{ActiveModel, Entity, Model};
use crate::bank_account_linking::constants::GO_CARDLESS_PROVIDER;
use crate::bank_account_linking::institutions::Institution;
use crate::error::app_error::{AppError, AppResult};
use crate::initializers::context::try_get_global_app_context;
use crate::models::_entities::external_bank_institutions::Column;
use crate::services::snowflake_generator::SnowflakeGenerator;
use crate::workers::external_bank_institutions::crud::deleted::{
    ExternalBankInstitutionDeleted, ExternalBankInstitutionDeletedArgs,
};
use crate::workers::external_bank_institutions::crud::insert::{
    ExternalBankInstitutionInserted, ExternalBankInstitutionInsertedArgs,
};
use crate::workers::external_bank_institutions::crud::update::{
    ExternalBankInstitutionUpdated, ExternalBankInstitutionUpdatedArgs,
};
use chrono::Utc;
use loco_rs::prelude::{AppContext, BackgroundWorker};
use sea_orm::entity::prelude::*;
use sea_orm::{DbBackend, Set, Statement, TransactionTrait, Unchanged};
use tracing::error;

pub type ExternalBankInstitutions = Entity;

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> std::result::Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if !self.updated_at.is_set() {
            self.updated_at = Set(Utc::now().into());
        }

        if insert {
            self.created_at = Set(Utc::now().into());
        }

        Ok(self)
    }

    async fn after_save<C>(
        model: <Self::Entity as EntityTrait>::Model,
        _: &C,
        insert: bool,
    ) -> Result<<Self::Entity as EntityTrait>::Model, DbErr>
    where
        C: ConnectionTrait,
    {
        if let Some(ctx) = try_get_global_app_context() {
            match insert {
                true => send_inserted_event(ctx, model.id).await,
                false => send_updated_event(ctx, model.id).await,
            }
        }

        Ok(model)
    }

    async fn after_delete<C>(self, _db: &C) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if let Some(ctx) = try_get_global_app_context() {
            send_deleted_event(ctx, self.id.clone().unwrap()).await
        }

        Ok(self)
    }
}

async fn send_updated_event(ctx: &AppContext, id: i64) {
    if let Err(err) = try_send_updated_event(ctx, id).await {
        error!("Error occurred while trying to send updated event. Error: {}", err)
    }
}

async fn try_send_updated_event(ctx: &AppContext, id: i64) -> AppResult<()> {
    Ok(ExternalBankInstitutionUpdated::perform_later(ctx, ExternalBankInstitutionUpdatedArgs { id }).await?)
}

async fn send_inserted_event(ctx: &AppContext, id: i64) {
    if let Err(err) = try_send_inserted_event(ctx, id).await {
        error!("Error occurred while trying to send inserted event. Error: {}", err)
    }
}

async fn try_send_inserted_event(ctx: &AppContext, id: i64) -> AppResult<()> {
    Ok(ExternalBankInstitutionInserted::perform_later(ctx, ExternalBankInstitutionInsertedArgs { id }).await?)
}

async fn send_deleted_event(ctx: &AppContext, id: i64) {
    if let Err(err) = try_send_deleted_event(ctx, id).await {
        error!("Error occurred while trying to send deleted event. Error: {}", err)
    }
}

async fn try_send_deleted_event(ctx: &AppContext, id: i64) -> AppResult<()> {
    Ok(ExternalBankInstitutionDeleted::perform_later(ctx, ExternalBankInstitutionDeletedArgs { id }).await?)
}

// implement your read-oriented logic here
impl Model {}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {
    pub async fn add_or_update_from_go_cardless(
        db: &DatabaseConnection,
        snowflake_generator: &SnowflakeGenerator,
        institution: Institution,
    ) -> AppResult<Model> {
        let external_id = institution.id.clone();
        let max_access_valid_for_days = institution.max_access_valid_for_days.parse::<i32>().ok();

        let mut model = ActiveModel {
            id: Default::default(),
            external_id: Set(institution.id),
            provider: Set(GO_CARDLESS_PROVIDER.to_string()),
            name: Set(institution.name),
            bic: Set(institution.bic),
            countries: Set(institution.countries),
            logo_link: Set(institution.logo),
            access_valid_for_days: Set(max_access_valid_for_days),
            created_at: Default::default(),
            updated_at: Default::default(),
        };

        match Self::find_by_external_id(db, external_id.as_str(), GO_CARDLESS_PROVIDER).await? {
            Some(found_model) => {
                model.id = Unchanged(found_model.id);
                model.created_at = Unchanged(found_model.created_at);

                Ok(model.update(db).await?)
            }
            None => {
                model.id = Set(snowflake_generator.next_id()?);
                model.created_at = Set(Utc::now().into());

                Ok(model.insert(db).await?)
            }
        }
    }

    pub async fn find_by_external_id(
        db: &DatabaseConnection,
        external_id: &str,
        provider: &str,
    ) -> AppResult<Option<Model>> {
        Ok(Entity::find()
            .filter(Column::ExternalId.eq(external_id))
            .filter(Column::Provider.eq(provider))
            .one(db)
            .await?)
    }

    pub async fn exist_by_external_id(db: &DatabaseConnection, external_id: &str, provider: &str) -> AppResult<bool> {
        Ok(Self::count_by_external_id(db, external_id, provider).await? > 0)
    }

    pub async fn count_by_external_id(db: &DatabaseConnection, external_id: &str, provider: &str) -> AppResult<u64> {
        Ok(Entity::find()
            .filter(Column::ExternalId.eq(external_id))
            .filter(Column::Provider.eq(provider))
            .count(db)
            .await?)
    }

    pub async fn find_unknown_institutions(
        db: &DatabaseConnection,
        external_ids: Vec<String>,
        provider: String,
    ) -> AppResult<Vec<Model>> {
        if external_ids.is_empty() {
            return Self::find_all_by_provider(db, provider.as_str()).await;
        }

        db.transaction::<_, _, AppError>(|txn| {
            Box::pin(async move {
                // 1. Create new temporary table
                let stmt = Statement::from_string(
                    DbBackend::Postgres,
                    "CREATE TEMPORARY TABLE external_bank_institutions_temp_external_ids (external_id TEXT);",
                );
                txn.execute(stmt).await?;

                let insert_values = external_ids
                    .iter()
                    .map(|id| format!("('{}')", id))
                    .collect::<Vec<_>>()
                    .join(", ");

                // 2. Insert Data into temp table
                let insert_into_temp_table = Statement::from_string(
                    DbBackend::Postgres,
                    format!(
                        "INSERT INTO external_bank_institutions_temp_external_ids (external_id) VALUES {}",
                        insert_values
                    ),
                );

                txn.execute(insert_into_temp_table).await?;

                // 2. Perform the LEFT JOIN query
                let query = Statement::from_string(
                    DbBackend::Postgres,
                    format!(
                        "
                    SELECT e.*
                    FROM {} e
                    LEFT JOIN external_bank_institutions_temp_external_ids t ON e.{} = t.external_id
                    WHERE t.external_id IS NULL
                    AND e.provider = '{}';
                ",
                        Entity.table_name(),
                        Column::ExternalId.to_string(),
                        provider
                    ),
                );

                let entities: Vec<Model> = Entity::find().from_raw_sql(query).all(txn).await?;

                // 4. Explicitly drop the temp table
                let drop_temp_table = Statement::from_string(
                    DbBackend::Postgres,
                    "DROP TABLE external_bank_institutions_temp_external_ids",
                );
                txn.execute(drop_temp_table).await?;

                Ok(entities)
            })
        })
        .await
        .map_err(|err| {
            error!("An unknown transaction error occurred. Error: {}", err.to_string());

            AppError::GeneralInternalServerError(err.to_string())
        })
    }

    pub async fn find_all_by_provider(db: &impl ConnectionTrait, provider: &str) -> AppResult<Vec<Model>> {
        Ok(Entity::find().filter(Column::Provider.eq(provider)).all(db).await?)
    }

    pub async fn count_all(db: &DatabaseConnection) -> AppResult<u64> {
        Ok(Entity::find().count(db).await?)
    }
}
