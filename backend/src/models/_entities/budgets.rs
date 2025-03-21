//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.3

use super::sea_orm_active_enums::BudgetType;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "budgets")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub criteria_id: i64,
    pub r#type: BudgetType,
    pub current_amount: i64,
    pub amount: i64,
    #[sea_orm(column_type = "Text")]
    pub cron: String,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
    pub map_all: bool,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::budget_criteria::Entity",
        from = "Column::CriteriaId",
        to = "super::budget_criteria::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    BudgetCriteria,
}

impl Related<super::budget_criteria::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BudgetCriteria.def()
    }
}
