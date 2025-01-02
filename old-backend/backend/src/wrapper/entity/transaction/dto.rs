use std::sync::Arc;

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use utoipa::ToSchema;
use validator::Validate;

use utility::snowflake::entity::Snowflake;

use crate::api::error::api::ApiError;
use crate::api::routes::transaction::check_transaction_permissions;
use crate::wrapper::entity::account::Account;
use crate::wrapper::entity::budget::validation::budget_exists;
use crate::wrapper::entity::budget::Budget;
use crate::wrapper::entity::currency::Currency;
use crate::wrapper::entity::transaction::template::TransactionTemplate;
use crate::wrapper::types::phantom::Phantom;

// TODO move source_id and destination_id into an enum and add validation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Validate, ToSchema)]
pub(crate) struct TransactionDTO {
    pub(crate) source_id: Option<Phantom<Account>>,
    pub(crate) destination_id: Option<Phantom<Account>>,
    pub(crate) amount: i64,
    pub(crate) currency_id: Phantom<Currency>,
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    #[validate(custom(function = "budget_exists"))]
    pub(crate) budget_id: Option<Phantom<Budget>>,
    #[serde(with = "time::serde::rfc3339")]
    pub(crate) executed_at: OffsetDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Validate, ToSchema)]
pub(crate) struct TransactionFromTemplate {
    pub(crate) template_id: Phantom<TransactionTemplate>,
    #[serde(with = "time::serde::rfc3339")]
    pub(crate) executed_at: OffsetDateTime,
}

impl TransactionDTO {
    pub(crate) async fn from_template(
        template: Arc<TransactionTemplate>,
        executed_at: OffsetDateTime,
    ) -> Result<Self, ApiError> {
        Ok(Self {
            source_id: template.source_id.clone(),
            destination_id: template.destination_id.clone(),
            amount: template.amount,
            currency_id: template.currency_id.clone(),
            name: template.name.clone(),
            description: template.description.clone(),
            budget_id: template.budget_id.clone(),
            executed_at,
        })
    }

    pub(crate) async fn check_permissions(&self, user_id: Snowflake) -> Result<bool, ApiError> {
        check_transaction_permissions(&self.budget_id, &self.source_id, &self.destination_id, user_id).await
    }
}
