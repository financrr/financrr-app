use actix_web::dev::Payload;
use actix_web::web::Json;
use actix_web::FromRequest;
use futures_util::future::LocalBoxFuture;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use utility::snowflake::entity::Snowflake;

use crate::api::error::api::ApiError;
use crate::util::validation::{validate_currency_exists, validate_iban};
use crate::wrapper::entity::account::Account;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Validate, ToSchema)]
pub(crate) struct AccountDTO {
    #[validate(length(min = 1, max = 255))]
    pub(crate) name: String,
    #[validate(length(max = 25000))]
    pub(crate) description: Option<String>,
    #[validate(custom(function = validate_iban))]
    pub(crate) iban: Option<String>,
    pub(crate) original_balance: i64,
    pub(crate) currency_id: Snowflake,
}

impl FromRequest for AccountDTO {
    type Error = ApiError;
    type Future = LocalBoxFuture<'static, Result<Self, ApiError>>;

    fn from_request(req: &actix_web::HttpRequest, payload: &mut Payload) -> Self::Future {
        let fut = Json::<Self>::from_request(req, payload);
        Box::pin(async move {
            let fut = fut.await?;
            let dto = fut.into_inner();
            dto.validate()?;
            validate_currency_exists(dto.currency_id).await?;

            Ok(dto)
        })
    }
}

impl From<Account> for AccountDTO {
    fn from(value: Account) -> Self {
        Self {
            name: value.name,
            description: value.description,
            iban: value.iban,
            original_balance: value.original_balance,
            currency_id: value.currency_id.get_id(),
        }
    }
}

impl From<&Account> for AccountDTO {
    fn from(value: &Account) -> Self {
        Self::from(value.to_owned())
    }
}
