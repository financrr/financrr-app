use crate::api::error::api::ApiError;
use crate::event::lifecycle::transaction::{TransactionCreation, TransactionDeletion, TransactionUpdate};
use crate::event::GenericEvent;
use crate::wrapper::entity::budget::dto::BudgetDTO;
use crate::wrapper::entity::budget::Budget;

async fn update_budget_amount(budget: Budget, amount: i64) -> Result<(), ApiError> {
    let mut dto = BudgetDTO::from(&budget);
    dto.amount += amount;
    budget.to_owned().update(dto).await?;

    Ok(())
}

pub(crate) fn budget_listener() {
    TransactionCreation::subscribe(transaction_created);
    TransactionUpdate::subscribe(transaction_updated);
    TransactionDeletion::subscribe(transaction_deleted);
}

async fn transaction_created(event: TransactionCreation) -> Result<(), ApiError> {
    let transaction = event.transaction;
    if let Some(budget) = transaction.budget_id {
        update_budget_amount(budget.fetch_inner().await?, transaction.amount).await?;
    }

    Ok(())
}

async fn transaction_updated(event: TransactionUpdate) -> Result<(), ApiError> {
    let old_transaction = event.old_transaction;
    let new_transaction = event.new_transaction;
    if let Some(budget) = old_transaction.budget_id {
        update_budget_amount(budget.fetch_inner().await?, old_transaction.amount).await?;
    }
    if let Some(budget) = new_transaction.budget_id {
        update_budget_amount(budget.fetch_inner().await?, -new_transaction.amount).await?;
    }

    Ok(())
}

async fn transaction_deleted(event: TransactionDeletion) -> Result<(), ApiError> {
    let transaction = event.transaction;
    if let Some(budget) = transaction.budget_id {
        update_budget_amount(budget.fetch_inner().await?, -transaction.amount).await?;
    }

    Ok(())
}
