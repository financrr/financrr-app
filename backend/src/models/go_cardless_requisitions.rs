pub use super::_entities::go_cardless_requisitions::{ActiveModel, Entity, Model};
use crate::bank_account_linking::requisitions::Requisition;
use crate::error::app_error::{AppError, AppResult};
use crate::models::_entities::go_cardless_requisitions::Column;
use crate::models::{external_bank_institutions, go_cardless_enduser_agreements, users};
use crate::services::snowflake_generator::SnowflakeGenerator;
use chrono::{Duration, Utc};
use sea_orm::entity::prelude::*;
use sea_orm::{NotSet, Set};

pub const CLEAN_UP_TIME_HOURS: i64 = 24;

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
impl Model {
    pub async fn find_by_id(db: &DatabaseConnection, id: i64) -> AppResult<Option<Self>> {
        Ok(Entity::find().filter(Column::Id.eq(id)).one(db).await?)
    }

    pub async fn find_by_id_or_err(db: &DatabaseConnection, id: i64) -> AppResult<Self> {
        Self::find_by_id(db, id).await?.ok_or(AppError::EntityNotFound())
    }

    pub async fn find_by_user_id(db: &DatabaseConnection, user_id: i64) -> AppResult<Option<Self>> {
        Ok(Entity::find()
            .filter(Column::UsedAt.is_not_null())
            .filter(Column::UserId.eq(user_id))
            .one(db)
            .await?)
    }
}

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
            used_at: NotSet,
            created_at: Default::default(),
            updated_at: Default::default(),
        }
        .insert(db)
        .await?)
    }

    pub async fn clean_up(db: &DatabaseConnection) -> AppResult<u64> {
        let cutoff_time = Utc::now() - Duration::hours(CLEAN_UP_TIME_HOURS);

        Ok(Entity::delete_many()
            .filter(Column::CreatedAt.lt(cutoff_time))
            .filter(Column::UsedAt.is_null())
            .exec(db)
            .await?
            .rows_affected)
    }

    pub async fn update_used_at(mut self, db: &DatabaseConnection, used_at: chrono::DateTime<Utc>) -> AppResult<Model> {
        self.used_at = Set(Some(used_at.into()));

        Ok(self.update(db).await?)
    }
}

// implement your custom finders, selectors oriented logic here
impl Entity {}
