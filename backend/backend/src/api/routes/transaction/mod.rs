use utility::snowflake::entity::Snowflake;

use crate::api::error::api::ApiError;
use crate::wrapper::entity::account::Account;
use crate::wrapper::entity::budget::Budget;
use crate::wrapper::permission::{Permission, Permissions};
use crate::wrapper::types::phantom::{Identifiable, Phantom};

pub(crate) mod controller;
pub(crate) mod recurring;
pub(crate) mod template;
pub(crate) mod validation;

pub(crate) async fn check_transaction_permissions(
    budget_id: &Option<Phantom<Budget>>,
    source_id: &Option<Phantom<Account>>,
    destination_id: &Option<Phantom<Account>>,
    user_id: Snowflake,
) -> Result<bool, ApiError> {
    if let Some(budget) = budget_id {
        let budget = Budget::find_by_id(budget.get_id()).await?;
        budget.has_permission(user_id, Permissions::READ_WRITE).await?;
    }

    match (source_id, destination_id) {
        (Some(source), Some(destination)) => {
            let source_permissions = source.has_permission(user_id, Permissions::READ_WRITE).await?;
            let destination_permissions = destination.has_permission(user_id, Permissions::READ_WRITE).await?;

            Ok(source_permissions && destination_permissions)
        }
        (Some(source), None) => {
            let source_permissions = source.has_permission(user_id, Permissions::READ_WRITE).await?;
            Ok(source_permissions)
        }
        (None, Some(destination)) => {
            let destination_permissions = destination.has_permission(user_id, Permissions::READ_WRITE).await?;
            Ok(destination_permissions)
        }
        (None, None) => Ok(false),
    }
}
