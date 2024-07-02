use actix_web::dev::Payload;
use actix_web::web::Json;
use actix_web::{FromRequest, HttpRequest};
use futures_util::future::LocalBoxFuture;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use utility::snowflake::entity::Snowflake;

use crate::api::error::api::ApiError;
use crate::api::routes::transaction::check_transaction_permissions;
use crate::api::routes::transaction::validation::validate_transaction;
use crate::wrapper::entity::account::Account;
use crate::wrapper::entity::budget::Budget;
use crate::wrapper::entity::currency::Currency;
use crate::wrapper::types::phantom::Phantom;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
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

    pub(crate) async fn validate(&self) -> Result<(), ApiError> {
        validate_transaction(&self.budget_id, &self.source_id, &self.destination_id).await
    }
}

impl FromRequest for TransactionTemplateDTO {
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
