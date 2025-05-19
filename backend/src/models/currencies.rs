use super::_entities::currencies::{ActiveModel, Column, Entity, Model};
use crate::error::app_error::{AppError, AppResult};
use sea_orm::entity::prelude::*;

pub type Currencies = Entity;

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)

    async fn before_save<C>(mut self, _db: &C, insert: bool) -> loco_rs::Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if !self.updated_at.is_set() {
            self.updated_at = sea_orm::ActiveValue::Set(chrono::Utc::now().into());
        }

        if insert {
            self.created_at = sea_orm::ActiveValue::Set(chrono::Utc::now().into());
        }

        Ok(self)
    }
}

impl Model {
    pub const UNKNOWN_CURRENCY_ISO_CODE: &str = "XXX";

    pub async fn get_default_currency(db: &DbConn) -> AppResult<Self> {
        Entity::find()
            .filter(Column::IsoCode.eq(Self::UNKNOWN_CURRENCY_ISO_CODE.to_string()))
            .filter(Column::UserId.is_null())
            .one(db)
            .await?
            .ok_or_else(AppError::EntityNotFound)
    }

    pub async fn get_by_iso_code(db: &DbConn, code: &str) -> AppResult<Option<Self>> {
        Ok(Entity::find()
            .filter(Column::IsoCode.eq(code))
            .filter(Column::UserId.is_null())
            .one(db)
            .await?)
    }

    pub async fn get_by_iso_code_with_default(db: &DbConn, code: &str) -> AppResult<Self> {
        match Self::get_by_iso_code(db, code).await? {
            Some(currency) => Ok(currency),
            None => Self::get_default_currency(db).await,
        }
    }
}
