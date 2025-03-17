use crate::error::app_error::{AppError, AppResult};
use crate::error::app_error::{
    GeneralInternalServerErrorResponse, GeneralValidationErrorResponse, InvalidVerificationTokenResponse,
};
use crate::models::users;
use crate::models::users::Model;
use crate::services::snowflake_generator::SnowflakeGenerator;
use crate::services::user_verification::UserVerificationService;
use crate::utils::context::AdditionalAppContextMethods;
use crate::validation::user::validate_email_uniqueness;
use crate::views::user::UserResponse;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::post;
use axum::{Extension, Form, Json, debug_handler};
use loco_rs::app::AppContext;
use loco_rs::prelude::Routes;
use sea_orm::IntoActiveModel;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::{Validate, ValidateArgs};

pub const MIN_PASSWORD_LENGTH: u64 = 8;
pub const MAX_PASSWORD_LENGTH: u64 = 10240;

pub const MIN_NAME_LENGTH: u64 = 2;
pub const MAX_NAME_LENGTH: u64 = 512;

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct VerifyParams {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1))]
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct ForgotParams {
    #[validate(email)]
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct ResetParams {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1))]
    pub token: String,
    #[validate(length(min = "MIN_PASSWORD_LENGTH", max = "MAX_PASSWORD_LENGTH"))]
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
#[validate(context = AppContext)]
pub struct RegisterParams {
    #[validate(email)]
    #[validate(custom(function = "validate_email_uniqueness", use_context))]
    pub email: String,
    #[validate(length(min = "MIN_PASSWORD_LENGTH", max = "MAX_PASSWORD_LENGTH"))]
    pub password: String,
    #[validate(length(min = "MIN_NAME_LENGTH", max = "MAX_NAME_LENGTH"))]
    pub name: String,
}

/// Registers a new User
#[utoipa::path(post,
    path = "/api/v1/users/register",
    tag = "User",
    responses(
        (status = StatusCode::CREATED, description = "Successfully registered a new User.", content_type="application/json", body = UserResponse),
        GeneralValidationErrorResponse,
        GeneralInternalServerErrorResponse,
    )
)]
#[debug_handler]
async fn register(
    State(ctx): State<AppContext>,
    Extension(user_verification_service): Extension<UserVerificationService>,
    Extension(snowflake_generator): Extension<SnowflakeGenerator>,
    Json(params): Json<RegisterParams>,
) -> Result<(StatusCode, Json<UserResponse>), AppError> {
    params.validate_with_args(&ctx)?;

    let res = Model::create_with_password(&ctx.db, &snowflake_generator, &params).await?;
    let active_model = res.into_active_model();

    let model = user_verification_service
        .send_verification_email_or_verify_user(active_model)
        .await?;

    Ok((StatusCode::CREATED, Json(UserResponse::from(model))))
}

/// Verifies a User
///
/// This endpoint is only used if an Email Server was configured.
/// Otherwise, the User is automatically verified.
#[utoipa::path(post,
    path = "/api/v1/users/verify",
    tag = "User",
    responses(
        (status = StatusCode::OK, description = "Successfully verified a User.", content_type="application/json", body = UserResponse),
        InvalidVerificationTokenResponse,
        GeneralValidationErrorResponse,
        GeneralInternalServerErrorResponse,
    )
)]
#[debug_handler]
async fn verify(
    State(ctx): State<AppContext>,
    Form(params): Form<VerifyParams>,
) -> Result<(StatusCode, Json<UserResponse>), AppError> {
    params.validate()?;

    let user = Model::find_by_verification_token(&ctx.db, &params.email, &params.token).await?;
    match user {
        None => Err(AppError::InvalidVerificationToken()),
        Some(user) => {
            let active_model = user.into_active_model();
            let user = active_model.verified(&ctx.db).await?;

            Ok((StatusCode::OK, Json(UserResponse::from(user))))
        }
    }
}

/// Sends a forgot password email to the User
///
/// This endpoint only works if an Email Server was configured.
#[utoipa::path(post,
    path = "/api/v1/users/forgot",
    tag = "User",
    responses(
        (status = StatusCode::OK, description = "Successfully sent forgot password email when user exists."),
        GeneralValidationErrorResponse,
        GeneralInternalServerErrorResponse,
    )
)]
#[debug_handler]
async fn forgot_password(
    State(ctx): State<AppContext>,
    Extension(user_service): Extension<UserVerificationService>,
    Form(params): Form<ForgotParams>,
) -> AppResult<StatusCode> {
    params.validate()?;

    let Some(user) = Model::find_by_email(&ctx.db, &params.email).await? else {
        // Return success to not expose registered users.
        return Ok(StatusCode::OK);
    };

    if !ctx.is_mailer_enabled() {
        Err(AppError::EmailConfigurationMissing())?
    }

    user_service
        .send_forgot_password_email(user.into_active_model())
        .await?;

    Ok(StatusCode::OK)
}

/// Resets the password of a User
#[utoipa::path(post,
    path = "/api/v1/users/reset",
    tag = "User",
    responses(
        (status = StatusCode::OK, description = "Successfully reset password.", content_type="application/json", body = UserResponse),
        GeneralValidationErrorResponse,
        GeneralInternalServerErrorResponse,
    )
)]
#[debug_handler]
async fn reset_password(
    State(ctx): State<AppContext>,
    Form(params): Form<ResetParams>,
) -> AppResult<(StatusCode, Json<UserResponse>)> {
    let user = users::Model::find_by_reset_token(&ctx.db, &params.email, &params.token).await?;

    let user = user
        .into_active_model()
        .reset_password(&ctx.db, &params.password)
        .await?;

    Ok((StatusCode::OK, Json(UserResponse::from(user))))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/users")
        .add("/register", post(register))
        .add("/verify", post(verify))
        .add("/forgot", post(forgot_password))
        .add("/reset", post(reset_password))
}
