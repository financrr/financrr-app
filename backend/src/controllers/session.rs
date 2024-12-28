use crate::controllers::user::{MAX_PASSWORD_LENGTH, MIN_PASSWORD_LENGTH};
use crate::error::app_error::{
    AppError, AppResult, GeneralInternalServerErrorResponse, InvalidEmailOrPasswordResponse,
};
use crate::models::_entities::sessions;
use crate::models::users;
use crate::services::secret_generator::SecretGenerator;
use crate::services::snowflake_generator::SnowflakeGenerator;
use crate::views::session::SessionResponse;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::post;
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

#[utoipa::path(post,
    path = "/api/v1/sessions/create",
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

    if !user.verify_password(&params.password) {
        return Err(AppError::InvalidEmailOrPassword())?;
    }

    let session =
        sessions::Model::create_session(&ctx.db, &snowflake_generator, &secret_generator, &user, &params).await?;

    Ok((StatusCode::CREATED, Json(SessionResponse::from((session, user)))))
}

pub fn routes() -> Routes {
    Routes::new().add("/create", post(create))
}
