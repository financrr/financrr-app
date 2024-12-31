use crate::controllers::user::{MAX_PASSWORD_LENGTH, MIN_PASSWORD_LENGTH};
use crate::error::app_error::{
    AppError, AppResult, GeneralInternalServerErrorResponse, InvalidBearerTokenResponse, InvalidEmailOrPasswordResponse,
};
use crate::middlewares::authentication::Authenticated;
use crate::models::_entities::sessions;
use crate::models::users;
use crate::services::secret_generator::SecretGenerator;
use crate::services::snowflake_generator::SnowflakeGenerator;
use crate::views::session::SessionResponse;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{debug_handler, Extension, Json};
use loco_rs::app::AppContext;
use loco_rs::controller::Routes;
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct CreateSessionParams {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = "MIN_PASSWORD_LENGTH", max = "MAX_PASSWORD_LENGTH"))]
    pub password: String,
    pub name: Option<String>,
    pub user_agent: Option<String>,
}

/// Login/Create a new Session.
///
/// This endpoint is used to create a new Session for a User.
/// Returns a new Session with a Bearer Token that can be used to authenticate the User.
#[utoipa::path(post,
    path = "/api/v1/sessions",
    tag = "Session",
    responses(
        (status = StatusCode::CREATED, description = "Successfully created a new Session.", content_type="application/json", body = SessionResponse),
        InvalidEmailOrPasswordResponse,
        GeneralInternalServerErrorResponse,
    ),
)]
#[debug_handler]
async fn create(
    State(ctx): State<AppContext>,
    Extension(snowflake_generator): Extension<SnowflakeGenerator>,
    Extension(secret_generator): Extension<SecretGenerator>,
    Json(params): Json<CreateSessionParams>,
) -> AppResult<(StatusCode, Json<SessionResponse>)> {
    params.validate()?;
    let user = users::Model::find_by_email(&ctx.db, &params.email).await?;

    let user = match user {
        None => return Err(AppError::InvalidEmailOrPassword())?,
        Some(user) => user,
    };

    if user.email_verified_at.is_none() {
        return Err(AppError::EmailNotVerified())?;
    }

    if !user.verify_password(&params.password) {
        return Err(AppError::InvalidEmailOrPassword())?;
    }

    let session =
        sessions::Model::create_session(&ctx.db, &snowflake_generator, &secret_generator, &user, &params).await?;

    Ok((StatusCode::CREATED, Json(SessionResponse::from((session, user)))))
}

/// Retrieve the current Session.
#[utoipa::path(get,
    path = "/api/v1/sessions/current",
    tag = "Session",
    responses(
        (status = StatusCode::OK, description = "Successfully retrieved the current Session.", content_type="application/json", body = SessionResponse),
        GeneralInternalServerErrorResponse,
        InvalidBearerTokenResponse,
    ),
    security(
        ("bearer_token" = [])
    ),
)]
#[debug_handler]
async fn current(
    State(ctx): State<AppContext>,
    Authenticated(session): Authenticated<sessions::Model>,
) -> AppResult<(StatusCode, Json<SessionResponse>)> {
    let user = users::Model::find_by_id(&ctx.db, session.user_id).await?;

    Ok((StatusCode::OK, Json(SessionResponse::from((session, user)))))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/sessions")
        .add("/", post(create))
        .add("/current", get(current))
}
