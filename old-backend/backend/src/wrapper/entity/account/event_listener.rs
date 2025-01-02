use crate::api::error::api::ApiError;
use crate::event::lifecycle::transaction::{TransactionCreation, TransactionDeletion, TransactionUpdate};
use crate::event::GenericEvent;
use crate::wrapper::entity::account::dto::AccountDTO;
use crate::wrapper::entity::account::Account;

async fn update_account_balance(account: Account, amount: i64) -> Result<(), ApiError> {
    let dto = AccountDTO::from(&account);
    let balance = account.balance + amount;
    account.update_with_balance(dto, balance).await?;

    Ok(())
}

pub(crate) fn account_listener() {
    TransactionCreation::subscribe(transaction_created);
    TransactionUpdate::subscribe(transaction_updated);
    TransactionDeletion::subscribe(transaction_deleted);
}

async fn transaction_created(event: TransactionCreation) -> Result<(), ApiError> {
    if let Some(source) = event.transaction.source_id {
        update_account_balance(source.fetch_inner().await?, -event.transaction.amount).await?;
    }
    if let Some(destination) = event.transaction.destination_id {
        update_account_balance(destination.fetch_inner().await?, event.transaction.amount).await?;
    }

    Ok(())
}

async fn transaction_updated(event: TransactionUpdate) -> Result<(), ApiError> {
    let old_transaction = event.old_transaction;
    let new_transaction = event.new_transaction;

    // Reverse the effect of the old transaction
    if let Some(source) = old_transaction.source_id {
        update_account_balance(source.fetch_inner().await?, old_transaction.amount).await?;
    }
    if let Some(destination) = old_transaction.destination_id {
        update_account_balance(destination.fetch_inner().await?, -old_transaction.amount).await?;
    }

    // Apply the new transaction
    if let Some(source) = new_transaction.source_id {
        update_account_balance(source.fetch_inner().await?, -new_transaction.amount).await?;
    }
    if let Some(destination) = new_transaction.destination_id {
        update_account_balance(destination.fetch_inner().await?, new_transaction.amount).await?;
    }

    Ok(())
}

async fn transaction_deleted(event: TransactionDeletion) -> Result<(), ApiError> {
    let transaction = event.transaction;
    if let Some(source) = transaction.source_id {
        update_account_balance(source.fetch_inner().await?, transaction.amount).await?;
    }
    if let Some(destination) = transaction.destination_id {
        update_account_balance(destination.fetch_inner().await?, -transaction.amount).await?;
    }

    Ok(())
}
