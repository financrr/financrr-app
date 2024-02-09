use crate::api::error::ApiError;
use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpRequest};
use actix_web_validator::Json;
use futures_util::future::LocalBoxFuture;
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CurrencyDTO {
	#[validate(length(min = 1, max = 255))]
	pub name: String,
	#[validate(length(min = 1, max = 255))]
	pub symbol: String,
	#[validate(length(min = 1, max = 3))]
	pub iso_code: String,
	#[validate(range(min = 0, max = 10))]
	pub decimal_places: i32,
}

impl FromRequest for CurrencyDTO {
	type Error = ApiError;
	type Future = LocalBoxFuture<'static, Result<Self, ApiError>>;

	fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
		let fut = Json::<Self>::from_request(req, payload);
		Box::pin(async move {
			let dto = fut.await?.into_inner();

			Ok(dto)
		})
	}
}
