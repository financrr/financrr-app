//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0-rc.4

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
    #[sea_orm(column_type = "Text", nullable)]
    pub display_name: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub password: String,
    pub created_at: TimeDateTimeWithTimeZone,
    pub is_admin: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::budget::Entity")]
    Budget,
    #[sea_orm(has_many = "super::currency::Entity")]
    Currency,
    #[sea_orm(has_many = "super::permissions::Entity")]
    Permissions,
    #[sea_orm(has_many = "super::session::Entity")]
    Session,
}

impl Related<super::budget::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Budget.def()
    }
}

impl Related<super::currency::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Currency.def()
    }
}

impl Related<super::permissions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Permissions.def()
    }
}

impl Related<super::session::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Session.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn verify_password(&self, password: &[u8]) -> Result<bool, EntityError> {
        argon2::verify_encoded(&self.password, password).map_err(EntityError::HashingFailed)
    }
}

impl ActiveModel {
    pub fn register(
        username: String,
        email: Option<String>,
        display_name: Option<String>,
        password: String,
    ) -> Result<Self, EntityError> {
        let hashed_password = hash_string(&password)?;

        Ok(Self {
            id: Default::default(),
            username: Set(username),
            email: Set(email),
            display_name: Set(display_name),
            password: Set(hashed_password),
            created_at: Set(get_now()),
            is_admin: Set(false),
        })
    }
}

impl Entity {
    pub fn find_by_username(username: &str) -> Select<Self> {
        Self::find().filter(Column::Username.eq(username.to_string()))
    }
}
