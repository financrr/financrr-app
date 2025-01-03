use utoipa::IntoResponses;

use crate::api::error::api::ApiError;

#[derive(IntoResponses)]
#[response(
    status = 401,
    description = "Unauthorized. Can be caused by: invalid credentials, expired token, missing token, malformed Authorization header, etc.",
    content_type = "application/json"
)]
#[allow(dead_code)]
pub(crate) struct Unauthorized(#[to_schema] ApiError);

#[derive(IntoResponses)]
#[response(status = 400, description = "Provided data isn't valid.", content_type = "application/json")]
#[allow(dead_code)]
pub(crate) struct ValidationError(#[to_schema] ApiError);

#[derive(IntoResponses)]
#[response(
    status = 500,
    description = "Internal server error. Like DbError, HashingError, etc.",
    content_type = "application/json"
)]
#[allow(dead_code)]
pub(crate) struct InternalServerError(#[to_schema] ApiError);

#[derive(IntoResponses)]
#[response(status = 404, description = "Resource not found.", content_type = "application/json")]
#[allow(dead_code)]
pub(crate) struct ResourceNotFound(#[to_schema] ApiError);
