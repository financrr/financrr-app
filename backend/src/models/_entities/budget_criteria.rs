//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.7

use super::sea_orm_active_enums::FilterTransactionType;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "budget_criteria")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub all_categories: bool,
    pub all_tags: bool,
    pub all_external_bank_accounts: bool,
    pub all_bank_accounts: bool,
    pub transaction_type: FilterTransactionType,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::budget_criteria_bank_accounts::Entity")]
    BudgetCriteriaBankAccounts,
    #[sea_orm(has_many = "super::budget_criteria_categories::Entity")]
    BudgetCriteriaCategories,
    #[sea_orm(has_many = "super::budget_criteria_external_bank_accounts::Entity")]
    BudgetCriteriaExternalBankAccounts,
    #[sea_orm(has_many = "super::budget_criteria_tags::Entity")]
    BudgetCriteriaTags,
    #[sea_orm(has_many = "super::budgets::Entity")]
    Budgets,
}

impl Related<super::budget_criteria_bank_accounts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BudgetCriteriaBankAccounts.def()
    }
}

impl Related<super::budget_criteria_categories::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BudgetCriteriaCategories.def()
    }
}

impl Related<super::budget_criteria_external_bank_accounts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BudgetCriteriaExternalBankAccounts.def()
    }
}

impl Related<super::budget_criteria_tags::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BudgetCriteriaTags.def()
    }
}

impl Related<super::budgets::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Budgets.def()
    }
}

impl Related<super::bank_accounts::Entity> for Entity {
    fn to() -> RelationDef {
        super::budget_criteria_bank_accounts::Relation::BankAccounts.def()
    }
    fn via() -> Option<RelationDef> {
        Some(
            super::budget_criteria_bank_accounts::Relation::BudgetCriteria
                .def()
                .rev(),
        )
    }
}

impl Related<super::categories::Entity> for Entity {
    fn to() -> RelationDef {
        super::budget_criteria_categories::Relation::Categories.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::budget_criteria_categories::Relation::BudgetCriteria.def().rev())
    }
}

impl Related<super::external_bank_accounts::Entity> for Entity {
    fn to() -> RelationDef {
        super::budget_criteria_external_bank_accounts::Relation::ExternalBankAccounts.def()
    }
    fn via() -> Option<RelationDef> {
        Some(
            super::budget_criteria_external_bank_accounts::Relation::BudgetCriteria
                .def()
                .rev(),
        )
    }
}

impl Related<super::tags::Entity> for Entity {
    fn to() -> RelationDef {
        super::budget_criteria_tags::Relation::Tags.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::budget_criteria_tags::Relation::BudgetCriteria.def().rev())
    }
}
