use crate::error::app_error::{
    AppError, AppResult, GeneralInternalServerErrorResponse, NotFoundResponse, UnauthorizedResponse,
};
use crate::middlewares::authentication::Authenticated;
use crate::models::users::users;
use crate::models::{external_bank_institutions, go_cardless_enduser_agreements, go_cardless_requisitions};
use crate::services::bank_linking_data::BankLinkingData;
use crate::services::custom_configs::base::CustomConfig;
use crate::services::snowflake_generator::SnowflakeGenerator;
use crate::types::snowflake::Snowflake;
use crate::views::go_cardless::CreateLinkingLinkResponse;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::post;
use axum::{Extension, Json};
use loco_rs::app::AppContext;
use loco_rs::prelude::Routes;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateLinkingLink {
    pub external_bank_institution_id: Snowflake,
}
/// Creates or gets you a linking link.
///
/// With this link the user can link their Bank Account to financrr.
#[utoipa::path(post,
    path = "/create-linking-link",
    context_path = "/api/v1/go-cardless",
    tag = "GoCardless",
    responses (
        (status = StatusCode::CREATED, description = "Successfully created a linking link.", content_type="application/json", body = CreateLinkingLinkResponse),
        NotFoundResponse,
        GeneralInternalServerErrorResponse,
        UnauthorizedResponse
    ),
    security(
        ("bearer_token" = [])
    ),
)]
// #[axum::debug_handler]
async fn create_linking_link(
    State(ctx): State<AppContext>,
    Authenticated(user): Authenticated<users::Model>,
    Extension(bank_linking_data): Extension<BankLinkingData>,
    Extension(snowflake_generator): Extension<SnowflakeGenerator>,
    Extension(custom_config): Extension<CustomConfig>,
    Json(data): Json<CreateLinkingLink>,
) -> AppResult<(StatusCode, Json<CreateLinkingLinkResponse>)> {
    let go_cardless_client = bank_linking_data
        .get_go_cardless_client()
        .ok_or(AppError::ConfigurationError(
            "GoCardless client is not configured!".to_string(),
        ))?;
    let bank_linking_config = custom_config
        .bank_data_linking
        .as_ref()
        .ok_or(AppError::ConfigurationError(
            "Bank linking config is not configured!".to_string(),
        ))?;

    // TODO check if link already exist and use that one
    let institution = external_bank_institutions::Entity::find_by_id(&ctx.db, data.external_bank_institution_id.id)
        .await?
        .ok_or(AppError::EntityNotFound())?;
    let agreement = go_cardless_enduser_agreements::Entity::find_by_external_bank_institution(
        &ctx.db,
        data.external_bank_institution_id.id,
    )
    .await?
    .ok_or(AppError::EntityNotFound())?;

    let response = go_cardless_client
        .create_requisition(
            bank_linking_config.redirect_url.as_str(),
            institution.external_id.as_str(),
            agreement.external_id.as_str(),
        )
        .await?;

    let requisition = go_cardless_requisitions::ActiveModel::from_api_response(
        &ctx.db,
        &snowflake_generator,
        &agreement,
        &institution,
        &user,
        response,
    )
    .await?;

    Ok((
        StatusCode::CREATED,
        Json(CreateLinkingLinkResponse { link: requisition.link }),
    ))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/go-cardless")
        .add("/create-linking-link", post(create_linking_link))
}
