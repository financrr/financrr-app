use actix_web::http::StatusCode;
use itertools::{Either, Itertools};

use crate::api::error::api::ApiError;

pub(crate) fn handle_async_result_vec<T>(results: Vec<Result<T, ApiError>>) -> Result<Vec<T>, ApiError> {
    let (entities, errors): (Vec<_>, Vec<_>) = results.into_iter().partition_map(|result| match result {
        Ok(entities) => Either::Left(entities),
        Err(error) => Either::Right(error),
    });

    if !errors.is_empty() {
        return Err(ApiError::from_error_vec(errors, StatusCode::INTERNAL_SERVER_ERROR));
    }

    Ok(entities)
}
