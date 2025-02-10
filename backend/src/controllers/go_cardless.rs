use crate::error::app_error::{
    AppError, AppResult, ConfigurationErrorResponse, GeneralInternalServerErrorResponse,
    NoAccountLinkedWithGoCardlessResponse, NotFoundResponse, UnauthorizedResponse,
};
use crate::middlewares::authentication::Authenticated;
use crate::models::users::users;
use crate::models::{external_bank_institutions, go_cardless_enduser_agreements, go_cardless_requisitions};
use crate::services::bank_linking_data::BankLinkingData;
use crate::services::custom_configs::base::CustomConfig;
use crate::services::snowflake_generator::SnowflakeGenerator;
use crate::types::snowflake::Snowflake;
use crate::views::go_cardless::CreateLinkingLinkResponse;
use crate::workers::go_cardless::sync_account_data::{SyncAccountData, SyncAccountDataArgs};
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::post;
use axum::{Extension, Json};
use chrono::Utc;
use loco_rs::app::AppContext;
use loco_rs::prelude::{BackgroundWorker, Routes};
use sea_orm::IntoActiveModel;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateLinkingLink {
    pub external_bank_institution_id: Snowflake,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct StartSyncingData {
    pub requisition_id: Snowflake,
}

/// Creates or gets you a linking link.
///
/// With this link the user can link their Bank Account to financrr.
#[utoipa::path(post,
    path = "/create-linking-link",
    context_path = "/api/v1/go-cardless",
    tag = "GoCardless",
    responses(
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

    let requisition = match go_cardless_requisitions::Model::find_by_user_id(&ctx.db, user.id).await? {
        Some(req) => req,
        None => {
            let institution =
                external_bank_institutions::Entity::find_by_id(&ctx.db, data.external_bank_institution_id.id)
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

            go_cardless_requisitions::ActiveModel::from_api_response(
                &ctx.db,
                &snowflake_generator,
                &agreement,
                &institution,
                &user,
                response,
            )
            .await?
        }
    };

    Ok((
        StatusCode::CREATED,
        Json(CreateLinkingLinkResponse {
            id: Snowflake::from(requisition.id),
            link: requisition.link,
        }),
    ))
}

/// Starts the syncing process
#[utoipa::path(post,
    path = "/start-syncing-data",
    context_path = "/api/v1/go-cardless",
    tag = "GoCardless",
    responses(
        (status = StatusCode::OK, description = "Successfully started the syncing process.",),
        NotFoundResponse,
        UnauthorizedResponse,
        NoAccountLinkedWithGoCardlessResponse,
        ConfigurationErrorResponse
    ),
    security(
        ("bearer_token" = [])
    ),
)]
// #[axum::debug_handler]
async fn start_syncing_data(
    State(ctx): State<AppContext>,
    Authenticated(user): Authenticated<users::Model>,
    Extension(bank_linking_data): Extension<BankLinkingData>,
    Json(data): Json<StartSyncingData>,
) -> AppResult<StatusCode> {
    let go_cardless_client = bank_linking_data
        .get_go_cardless_client()
        .ok_or(AppError::ConfigurationError(
            "GoCardless client is not configured!".to_string(),
        ))?;

    let requisition_model = go_cardless_requisitions::Model::find_by_id_or_err(&ctx.db, data.requisition_id.id).await?;
    let requisition = go_cardless_client
        .get_requisition(requisition_model.external_id.as_str())
        .await?;

    match requisition.accounts.is_empty() {
        true => Err(AppError::NoAccountLinkedWithGoCardless()),
        false => {
            _ = requisition_model
                .into_active_model()
                .update_used_at(&ctx.db, Utc::now())
                .await?;
            Ok(())
        }
    }?;

    SyncAccountData::perform_later(&ctx, SyncAccountDataArgs { user_id: user.id }).await?;

    Ok(StatusCode::OK)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/go-cardless")
        .add("/create-linking-link", post(create_linking_link))
        .add("/start-syncing-data", post(start_syncing_data))
}
