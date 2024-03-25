//! `SeaORM` Entity. Generated by sea-orm-codegen 1.0.0-rc.2

use sea_orm::entity::prelude::*;
use sea_orm::{Condition, QuerySelect};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "currency")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    #[sea_orm(column_type = "Text")]
    pub symbol: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub iso_code: Option<String>,
    pub decimal_places: i32,
    pub user: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::account::Entity")]
    Account,
    #[sea_orm(has_many = "super::transaction::Entity")]
    Transaction,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::User",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    User,
}

impl Related<super::account::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Account.def()
    }
}

impl Related<super::transaction::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Transaction.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Entity {
    pub fn find_all_with_no_user() -> Select<Self> {
        Entity::find().filter(Column::User.is_null())
    }

    pub fn find_all_with_user(user_id: i32) -> Select<Self> {
        Entity::find().filter(Column::User.eq(user_id))
    }

    pub fn find_all_with_no_user_and_user(user_id: i32) -> Select<Self> {
        Entity::find().filter(Condition::any().add(Column::User.is_null()).add(Column::User.eq(user_id)))
    }

    pub fn count_all_with_no_user_and_user(user_id: i32) -> Select<Self> {
        Self::find_all_with_no_user_and_user(user_id).column(Column::Id)
    }

    pub fn find_by_id_with_no_user(id: i32) -> Select<Self> {
        Entity::find().filter(Column::Id.eq(id)).filter(Column::User.is_null())
    }

    pub fn find_by_id_related_with_user(id: i32, user_id: i32) -> Select<Self> {
        Entity::find().filter(Column::Id.eq(id)).filter(Column::User.eq(user_id))
    }

    pub fn find_by_id_include_user(id: i32, user_id: i32) -> Select<Self> {
        Entity::find()
            .filter(Column::Id.eq(id))
            .filter(Condition::any().add(Column::User.is_null()).add(Column::User.eq(user_id)))
    }
}