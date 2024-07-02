use std::sync::Arc;

use actix_web::dev::Payload;
use actix_web::web::Json;
use actix_web::{FromRequest, HttpRequest};
use futures_util::future::LocalBoxFuture;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use utoipa::ToSchema;
use validator::Validate;

use utility::snowflake::entity::Snowflake;

use crate::api::error::api::ApiError;
use crate::api::routes::transaction::check_transaction_permissions;
use crate::api::routes::transaction::validation::validate_transaction;
use crate::wrapper::entity::account::Account;
use crate::wrapper::entity::budget::Budget;
use crate::wrapper::entity::currency::Currency;
use crate::wrapper::entity::transaction::template::TransactionTemplate;
use crate::wrapper::types::phantom::Phantom;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Validate, ToSchema)]
pub(crate) struct TransactionDTO {
    pub(crate) source_id: Option<Phantom<Account>>,
    pub(crate) destination_id: Option<Phantom<Account>>,
    pub(crate) amount: i64,
    pub(crate) currency_id: Phantom<Currency>,
    pub(crate) name: String,
    pub(crate) description: Option<String>,
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

    async fn validate(&self) -> Result<(), ApiError> {
        validate_transaction(&self.budget_id, &self.source_id, &self.destination_id).await
    }
}

impl FromRequest for TransactionDTO {
    type Error = ApiError;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let json_fut = Json::<Self>::from_request(req, payload);
        Box::pin(async move {
            let dto = json_fut.await?;
            let dto = dto.into_inner();
            dto.validate().await?;

            Ok(dto)
        })
    }
}

impl FromRequest for TransactionFromTemplate {
    type Error = ApiError;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let json_fut = Json::<Self>::from_request(req, payload);
        Box::pin(async move {
            let dto = json_fut.await?;
            let dto = dto.into_inner();

            Ok(dto)
        })
    }
}
