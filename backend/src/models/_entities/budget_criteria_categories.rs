//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.7

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "budget_criteria_categories")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub budget_criteria_id: i64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub category_id: i64,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::budget_criteria::Entity",
        from = "Column::BudgetCriteriaId",
        to = "super::budget_criteria::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    BudgetCriteria,
    #[sea_orm(
        belongs_to = "super::categories::Entity",
        from = "Column::CategoryId",
        to = "super::categories::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Categories,
}

impl Related<super::budget_criteria::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BudgetCriteria.def()
    }
}

impl Related<super::categories::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Categories.def()
    }
}
