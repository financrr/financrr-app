use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpRequest};
use actix_web_validator::Json;
use futures_util::future::LocalBoxFuture;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::api::error::api::ApiError;
use crate::util::validation::{validate_password, validate_unique_username};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Validate, ToSchema)]
pub(crate) struct UserRegistration {
    #[validate(length(min = 1))]
    pub(crate) username: String,
    #[validate(email)]
    pub(crate) email: Option<String>,
    pub(crate) display_name: Option<String>,
    #[validate(custom = "validate_password")]
    pub(crate) password: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Validate, ToSchema)]
pub(crate) struct Credentials {
    #[validate(length(min = 1))]
    pub(crate) username: String,
    #[validate(length(min = 1))]
    pub(crate) password: String,
    #[validate(length(min = 1))]
    pub(crate) session_name: Option<String>,
}

impl FromRequest for UserRegistration {
    type Error = ApiError;
    type Future = LocalBoxFuture<'static, Result<Self, ApiError>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let registration = Json::<Self>::from_request(req, payload);
        Box::pin(async move {
            let registration = registration.await?;
            validate_unique_username(registration.username.as_str()).await?;

            Ok(registration.into_inner())
        })
    }
}