//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "budget_criteria_tags")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub budget_criteria_id: i64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub tag_id: i64,
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
        belongs_to = "super::tags::Entity",
        from = "Column::TagId",
        to = "super::tags::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Tags,
}

impl Related<super::budget_criteria::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BudgetCriteria.def()
    }
}

impl Related<super::tags::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tags.def()
    }
}
