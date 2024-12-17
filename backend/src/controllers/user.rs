use crate::error::app_error::AppError;
use crate::models::users::{Model, RegisterParams};
use crate::services::snowflake_generator::SnowflakeGenerator;
use crate::services::user_verification::UserVerificationService;
use crate::views::user::UserResponse;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::post;
use axum::{debug_handler, Extension, Json};
use loco_rs::app::AppContext;
use loco_rs::prelude::Routes;
use sea_orm::IntoActiveModel;
use serde::Deserialize;
use utoipa::ToSchema;
use validator::{Validate, ValidationError, ValidationErrors};

#[derive(Debug, Deserialize, ToSchema)]
pub struct VerifyParams {
    pub token: String,
}

/// Registers a new User.
#[utoipa::path(post,
    path = "/api/v1/users/register",
    tag = "User",
    responses(
        (status = StatusCode::CREATED, description = "Successfully registered a new User.", content_type="application/json", body = UserResponse),
        (status = StatusCode::BAD_REQUEST, description = "Validation error.", content_type="application/json", body = AppError),
    )
)]
#[debug_handler]
async fn register(
    State(ctx): State<AppContext>,
    Extension(user_verification_service): Extension<UserVerificationService>,
    Extension(snowflake_generator): Extension<SnowflakeGenerator>,
    Json(params): Json<RegisterParams>,
) -> Result<(StatusCode, Json<UserResponse>), AppError> {
    params.validate()?;
    if !Model::is_email_unique(&ctx.db, &params.email).await? {
        let mut errors = ValidationErrors::new();
        errors.add("email", ValidationError::new("Email already exists."));

        return Err(errors.into());
    }

    let res = Model::create_with_password(&ctx.db, &snowflake_generator, &params).await?;
    let active_model = res.into_active_model();

    let model = user_verification_service
        .send_verification_email_or_verify_user(active_model)
        .await?;

    Ok((StatusCode::CREATED, Json(UserResponse::from(model))))
}

/// Verifies a User with the given token.
#[utoipa::path(post,
    path = "/api/v1/users/verify",
    tag = "User",
    responses(
        (status = StatusCode::OK, description = "Successfully verified a User.", content_type="application/json", body = UserResponse),
        (status = StatusCode::BAD_REQUEST, description = "Invalid verification token.", content_type="application/json", body = AppError),
    )
)]
#[debug_handler]
async fn verify(
    State(ctx): State<AppContext>,
    Json(params): Json<VerifyParams>,
) -> Result<(StatusCode, Json<UserResponse>), AppError> {
    let user = Model::find_by_verification_token(&ctx.db, &params.token).await?;
    match user {
        None => Err(AppError::InvalidVerificationToken()),
        Some(user) => {
            let active_model = user.into_active_model();
            let user = active_model.verified(&ctx.db).await?;

            Ok((StatusCode::OK, Json(UserResponse::from(user))))
        }
    }
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/users")
        .add("/register", post(register))
        .add("/verify", post(verify))
}
