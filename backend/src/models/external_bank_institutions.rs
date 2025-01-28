pub use super::_entities::external_bank_institutions::{ActiveModel, Entity, Model};
use crate::bank_account_linking::constants::GO_CARDLESS_PROVIDER;
use crate::bank_account_linking::institutions::Institution;
use crate::error::app_error::{AppError, AppResult};
use crate::models::_entities::external_bank_institutions::Column;
use crate::services::snowflake_generator::SnowflakeGenerator;
use chrono::Utc;
use sea_orm::entity::prelude::*;
use sea_orm::sea_query::OnConflict;
use sea_orm::{DbBackend, IntoActiveModel, Set, Statement, TransactionTrait};
use tracing::log::warn;
use tracing::{error, info};

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
}

// implement your read-oriented logic here
impl Model {}

// implement your write-oriented logic here
impl ActiveModel {
    pub async fn delete_unknown_institutions(
        db: &DatabaseConnection,
        external_ids: Vec<String>,
        provider: String,
    ) -> AppResult<()> {
        let institutions = Entity::find_unknown_institutions(db, external_ids, provider).await?;

        info!("Deleting {} external institutions.", institutions.len());
        for institution in institutions {
            if let Err(err) = institution.into_active_model().delete(db).await {
                warn!("Could not delete external institution with err: {}", err);
            }
        }

        Ok(())
    }

    pub fn from_go_cardless(institution: Institution, snowflake_generator: &SnowflakeGenerator) -> AppResult<Self> {
        let id = snowflake_generator.next_id()?;
        let max_access_valid_for_days = institution.max_access_valid_for_days.parse::<i32>().ok();

        Ok(Self {
            id: Set(id),
            external_id: Set(institution.id),
            provider: Set(GO_CARDLESS_PROVIDER.to_string()),
            name: Set(institution.name),
            bic: Set(institution.bic),
            countries: Set(institution.countries),
            logo_link: Set(institution.logo),
            access_valid_for_days: Set(max_access_valid_for_days),
            created_at: Set(Utc::now().into()),
            updated_at: Set(Utc::now().into()),
        })
    }

    fn from_go_cardless_vec(institutions: Vec<Institution>, snowflake_generator: &SnowflakeGenerator) -> Vec<Self> {
        let mut successful_models = Vec::new();

        for institution in institutions {
            let external_id = institution.id.clone();
            match ActiveModel::from_go_cardless(institution, snowflake_generator) {
                Ok(active_model) => successful_models.push(active_model),
                Err(err) => warn!(
                    "Failed to convert institution to active model: {}. Error: {}",
                    external_id, err
                ),
            }
        }

        successful_models
    }
}

// implement your custom finders, selectors oriented logic here
impl Entity {
    pub async fn add_or_update_many_from_go_cardless(
        db: &DatabaseConnection,
        snowflake_generator: &SnowflakeGenerator,
        institutions: Vec<Institution>,
    ) -> AppResult<()> {
        let active_models = ActiveModel::from_go_cardless_vec(institutions, snowflake_generator);
        let on_conflict = OnConflict::columns([Column::ExternalId, Column::Provider])
            .update_columns([
                Column::Name,
                Column::Bic,
                Column::Countries,
                Column::LogoLink,
                Column::AccessValidForDays,
                Column::UpdatedAt,
            ])
            .to_owned();

        Self::insert_many(active_models)
            .on_empty_do_nothing()
            .on_conflict(on_conflict)
            .exec_without_returning(db)
            .await?;

        Ok(())
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
