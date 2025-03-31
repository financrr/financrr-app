pub use super::_entities::go_cardless_enduser_agreements::{ActiveModel, Entity, Model};
use crate::bank_account_linking::agreements::EndUserAgreement;
use crate::error::app_error::AppResult;
use crate::models::_entities::go_cardless_enduser_agreements::Column;
use crate::services::snowflake_generator::SnowflakeGenerator;
use chrono::Utc;
use sea_orm::Set;
use sea_orm::entity::prelude::*;

pub type GoCardlessEnduserAgreements = Entity;

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
impl Model {
    pub async fn find_by_external_institution_id(db: &DatabaseConnection, id: i64) -> AppResult<Option<Model>> {
        Ok(Entity::find()
            .filter(Column::ExternalBankInstitutionId.eq(id))
            .one(db)
            .await?)
    }
}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {
    pub async fn find_by_external_id(db: &DatabaseConnection, external_id: &str) -> AppResult<Option<Model>> {
        Ok(Self::find().filter(Column::ExternalId.eq(external_id)).one(db).await?)
    }

    pub async fn find_by_external_bank_institution(
        db: &DatabaseConnection,
        institution_id: i64,
    ) -> AppResult<Option<Model>> {
        Ok(Self::find()
            .filter(Column::ExternalBankInstitutionId.eq(institution_id))
            .one(db)
            .await?)
    }

    pub async fn create_from_api_response(
        db: &DatabaseConnection,
        snowflake_generator: &SnowflakeGenerator,
        data: &EndUserAgreement,
        external_bank_institutions_id: i64,
    ) -> AppResult<Model> {
        Ok(ActiveModel {
            id: Set(snowflake_generator.next_id()?),
            external_id: Set(data.id.clone()),
            external_bank_institution_id: Set(external_bank_institutions_id),
            max_historical_days: Set(data.max_historical_days as i32),
            access_valid_for_days: Set(data.access_valid_for_days as i32),
            created_at: Default::default(),
            updated_at: Default::default(),
        }
        .insert(db)
        .await?)
    }
}
