//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.7

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "budget_type")]
pub enum BudgetType {
    #[sea_orm(string_value = "resetting")]
    Resetting,
    #[sea_orm(string_value = "accumulating")]
    Accumulating,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "filter_transaction_type")]
pub enum FilterTransactionType {
    #[sea_orm(string_value = "all")]
    All,
    #[sea_orm(string_value = "contracts")]
    Contracts,
    #[sea_orm(string_value = "non-contracts")]
    NonContracts,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "transaction_type")]
pub enum TransactionType {
    #[sea_orm(string_value = "income")]
    Income,
    #[sea_orm(string_value = "expense")]
    Expense,
    #[sea_orm(string_value = "transfer")]
    Transfer,
}
