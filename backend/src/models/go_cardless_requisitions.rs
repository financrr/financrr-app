pub use super::_entities::go_cardless_requisitions::{ActiveModel, Entity, Model};
use crate::bank_account_linking::requisitions::Requisition;
use crate::error::app_error::AppResult;
use crate::models::{external_bank_institutions, go_cardless_enduser_agreements, users};
use crate::services::snowflake_generator::SnowflakeGenerator;
use chrono::Utc;
use sea_orm::entity::prelude::*;
use sea_orm::Set;

pub type GoCardlessRequisition = Entity;

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
    pub async fn from_api_response(
        db: &DatabaseConnection,
        snowflake_generator: &SnowflakeGenerator,
        agreement: &go_cardless_enduser_agreements::Model,
        external_bank_institutions: &external_bank_institutions::Model,
        user: &users::Model,
        response: Requisition,
    ) -> AppResult<Model> {
        Ok(ActiveModel {
            id: Set(snowflake_generator.next_id()?),
            external_id: Set(response.id),
            link: Set(response.link),
            agreement_id: Set(agreement.id),
            external_bank_institution_id: Set(external_bank_institutions.id),
            user_id: Set(user.id),
            created_at: Default::default(),
            updated_at: Default::default(),
        }
        .insert(db)
        .await?)
    }
}

// implement your custom finders, selectors oriented logic here
impl Entity {}
