use serde::{Deserialize, Serialize};
use utility::snowflake::entity::Snowflake;
use utoipa::ToSchema;
use validator::Validate;

use crate::api::error::api::ApiError;
use crate::api::routes::transaction::check_transaction_permissions;
use crate::wrapper::entity::account::Account;
use crate::wrapper::entity::budget::Budget;
use crate::wrapper::entity::currency::Currency;
use crate::wrapper::types::phantom::Phantom;

//TODO add transaction validation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Validate, ToSchema)]
pub(crate) struct TransactionTemplateDTO {
    pub(crate) source_id: Option<Phantom<Account>>,
    pub(crate) destination_id: Option<Phantom<Account>>,
    pub(crate) amount: i64,
    pub(crate) currency_id: Phantom<Currency>,
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) budget_id: Option<Phantom<Budget>>,
}

impl TransactionTemplateDTO {
    pub(crate) async fn check_permissions(&self, user_id: Snowflake) -> Result<bool, ApiError> {
        check_transaction_permissions(&self.budget_id, &self.source_id, &self.destination_id, user_id).await
    }
}
