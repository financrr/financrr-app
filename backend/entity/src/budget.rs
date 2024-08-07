//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0-rc.4

use sea_orm::entity::prelude::*;
use sea_orm::{Order, QueryOrder};
use serde::{Deserialize, Serialize};

use utility::snowflake::entity::Snowflake;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "budget")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub user: i64,
    pub amount: i64,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
    pub created_at: TimeDateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::transaction::Entity")]
    Transaction,
    #[sea_orm(has_many = "super::transaction_template::Entity")]
    TransactionTemplate,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::User",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    User,
}

impl Related<super::transaction::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Transaction.def()
    }
}

impl Related<super::transaction_template::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TransactionTemplate.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Entity {
    pub fn find_all_by_user_id(user_id: Snowflake) -> Select<Self> {
        Self::find().filter(Column::User.eq(user_id)).order_by(Column::Id, Order::Desc)
    }
}
