//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "bank_accounts")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    #[sea_orm(unique)]
    pub currency_id: i64,
    pub linked_back_account_id: Option<i64>,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub iban: Option<String>,
    pub balance: i64,
    pub original_balance: i64,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::budget_criteria_bank_accounts::Entity")]
    BudgetCriteriaBankAccounts,
    #[sea_orm(
        belongs_to = "super::currencies::Entity",
        from = "Column::CurrencyId",
        to = "super::currencies::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Currencies,
    #[sea_orm(
        belongs_to = "super::linked_back_accounts::Entity",
        from = "Column::LinkedBackAccountId",
        to = "super::linked_back_accounts::Column::Id",
        on_update = "NoAction",
        on_delete = "SetNull"
    )]
    LinkedBackAccounts,
    #[sea_orm(has_many = "super::transaction_parties::Entity")]
    TransactionParties,
}

impl Related<super::budget_criteria_bank_accounts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BudgetCriteriaBankAccounts.def()
    }
}

impl Related<super::currencies::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Currencies.def()
    }
}

impl Related<super::linked_back_accounts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LinkedBackAccounts.def()
    }
}

impl Related<super::transaction_parties::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TransactionParties.def()
    }
}

impl Related<super::budget_criteria::Entity> for Entity {
    fn to() -> RelationDef {
        super::budget_criteria_bank_accounts::Relation::BudgetCriteria.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::budget_criteria_bank_accounts::Relation::BankAccounts.def().rev())
    }
}
