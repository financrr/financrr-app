//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.3

use super::sea_orm_active_enums::BudgetType;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "budget_histories")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub budget_id: i64,
    pub budget_type: BudgetType,
    pub current_amount: i64,
    pub amount: i64,
    #[sea_orm(column_type = "Text")]
    pub cron: String,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
    pub map_all: bool,
    pub recorded_at: DateTimeWithTimeZone,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}