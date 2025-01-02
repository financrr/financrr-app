use actix_web::dev::Payload;
use actix_web::web::Json;
use actix_web::{FromRequest, HttpRequest};
use futures_util::future::LocalBoxFuture;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::api::error::api::ApiError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Validate, ToSchema)]
pub(crate) struct CurrencyDTO {
    #[validate(length(min = 1, max = 255))]
    pub(crate) name: String,
    #[validate(length(min = 1, max = 255))]
    pub(crate) symbol: String,
    #[validate(length(min = 1, max = 3))]
    pub(crate) iso_code: Option<String>,
    #[validate(range(min = 0, max = 10))]
    pub(crate) decimal_places: i32,
}

impl FromRequest for CurrencyDTO {
    type Error = ApiError;
    type Future = LocalBoxFuture<'static, Result<Self, ApiError>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let fut = Json::<Self>::from_request(req, payload);
        Box::pin(async move {
            let dto = fut.await?.into_inner();
            dto.validate()?;

            Ok(dto)
        })
    }
}
