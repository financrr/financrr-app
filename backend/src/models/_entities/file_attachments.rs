//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "file_attachments")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub globally_accessible: bool,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    #[sea_orm(column_type = "Text")]
    pub path: String,
    #[sea_orm(column_type = "Text")]
    pub r#type: String,
    pub size: i64,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::external_bank_accounts::Entity")]
    ExternalBankAccounts,
    #[sea_orm(has_many = "super::pending_transactions::Entity")]
    PendingTransactions,
    #[sea_orm(has_many = "super::recurring_transactions::Entity")]
    RecurringTransactions,
    #[sea_orm(has_many = "super::transaction_templates::Entity")]
    TransactionTemplates,
    #[sea_orm(has_many = "super::transactions::Entity")]
    Transactions,
}

impl Related<super::external_bank_accounts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ExternalBankAccounts.def()
    }
}

impl Related<super::pending_transactions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PendingTransactions.def()
    }
}

impl Related<super::recurring_transactions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RecurringTransactions.def()
    }
}

impl Related<super::transaction_templates::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TransactionTemplates.def()
    }
}

impl Related<super::transactions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Transactions.def()
    }
}