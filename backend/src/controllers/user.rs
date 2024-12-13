use crate::error::app_error::AppError;
use crate::models::users::{Model, RegisterParams};
use crate::services::snowflake_generator::SnowflakeGenerator;
use crate::services::user_verification::UserVerificationService;
use crate::views::user::{RegistrationResponse, UserResponse};
use axum::extract::State;
use axum::routing::post;
use axum::{debug_handler, Extension, Json};
use loco_rs::app::AppContext;
use loco_rs::prelude::Routes;
use sea_orm::IntoActiveModel;
use validator::{Validate, ValidationError, ValidationErrors};

#[utoipa::path(post, path = "/api/v1/users/register", tag = "User")]
#[debug_handler]
async fn register(
    State(ctx): State<AppContext>,
    Extension(user_verification_service): Extension<UserVerificationService>,
    Extension(snowflake_generator): Extension<SnowflakeGenerator>,
    Json(params): Json<RegisterParams>,
) -> Result<Json<UserResponse>, AppError> {
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

    Ok(Json(UserResponse::from(model)))
}

pub fn routes() -> Routes {
    Routes::new().prefix("/users").add("/register", post(register))
}
