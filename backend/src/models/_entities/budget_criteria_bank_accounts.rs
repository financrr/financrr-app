//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.4

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "budget_criteria_bank_accounts")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub budget_criteria_id: i64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub bank_account_id: i64,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bank_accounts::Entity",
        from = "Column::BankAccountId",
        to = "super::bank_accounts::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    BankAccounts,
    #[sea_orm(
        belongs_to = "super::budget_criteria::Entity",
        from = "Column::BudgetCriteriaId",
        to = "super::budget_criteria::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    BudgetCriteria,
}

impl Related<super::bank_accounts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BankAccounts.def()
    }
}

impl Related<super::budget_criteria::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BudgetCriteria.def()
    }
}
