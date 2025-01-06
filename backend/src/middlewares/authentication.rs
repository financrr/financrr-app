use crate::error::app_error::{AppError, AppResult};
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::HeaderMap;
use loco_rs::prelude::AppContext;
use std::future::Future;

const TOKEN_PREFIX: &str = "Bearer ";
const AUTH_HEADER: &str = "authorization";

pub trait Authenticate: Sized {
    fn find_by_api_key(ctx: &AppContext, api_key: &str) -> impl Future<Output = AppResult<Self>> + Send;
}

pub struct Authenticated<T: Authenticate>(pub T);

impl<T: Authenticate> FromRequestParts<AppContext> for Authenticated<T> {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &AppContext) -> AppResult<Self> {
        let api_key = extract_api_key_from_header(&parts.headers)?;

        let inner = T::find_by_api_key(state, &api_key).await?;

        Ok(Self(inner))
    }
}

fn extract_api_key_from_header(headers: &HeaderMap) -> AppResult<String> {
    Ok(headers
        .get(AUTH_HEADER)
        .ok_or_else(AppError::AuthHeaderMissing)?
        .to_str()
        .map_err(|err| AppError::InvalidHeaderValue(err.to_string()))?
        .strip_prefix(TOKEN_PREFIX)
        .ok_or_else(AppError::InvalidBearerToken)?
        .to_string())
}
