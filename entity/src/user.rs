//! `SeaORM` Entity. Generated by sea-orm-codegen 1.0.0-rc.1

use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};

use crate::error::EntityError;
use crate::utility::hashing::hash_string;
use crate::utility::time::get_now;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Text", unique)]
    pub username: String,
    #[sea_orm(column_type = "Text", nullable, unique)]
    pub email: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub password: String,
    pub created_at: TimeDateTimeWithTimeZone,
    pub is_admin: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::currency::Entity")]
    Currency,
    #[sea_orm(has_many = "super::user_account::Entity")]
    UserAccount,
}

impl Related<super::currency::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Currency.def()
    }
}

impl Related<super::user_account::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserAccount.def()
    }
}

impl Related<super::account::Entity> for Entity {
    fn to() -> RelationDef {
        super::user_account::Relation::Account.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::user_account::Relation::User.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn verify_password(&self, password: &[u8]) -> Result<bool, EntityError> {
        argon2::verify_encoded(&self.password, password).map_err(EntityError::HashingFailed)
    }
}

impl ActiveModel {
    pub fn register(username: String, email: Option<String>, password: String) -> Result<Self, EntityError> {
        let hashed_password = hash_string(&password)?;

        Ok(Self {
            id: Default::default(),
            username: Set(username),
            email: Set(email),
            password: Set(hashed_password),
            created_at: Set(get_now()),
            is_admin: Set(false),
        })
    }
}

impl Entity {
    pub fn find_by_username(username: String) -> Select<Self> {
        Self::find().filter(Column::Username.eq(username))
    }
}
