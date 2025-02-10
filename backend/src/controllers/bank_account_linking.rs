use crate::bank_account_linking::constants::DEFAULT_ACCESS_VALID_FOR;
use crate::error::app_error::{
    AppError, AppResult, GeneralInternalServerErrorResponse, NotFoundResponse, UnauthorizedResponse,
};
use crate::middlewares::authentication::Authenticated;
use crate::models::_entities::sessions;
use crate::models::external_bank_institutions::ExternalBankInstitutions;
use crate::models::go_cardless_enduser_agreements::{Entity, Model};
use crate::services::bank_linking_data::BankLinkingData;
use crate::services::snowflake_generator::SnowflakeGenerator;
use crate::types::snowflake::Snowflake;
use crate::views::bank_account_linking::StartLinkingProcessResponse;
use axum::extract::State;
use axum::routing::post;
use axum::{Extension, Json};
use loco_rs::app::AppContext;
use loco_rs::prelude::Routes;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub enum LinkingFlows {
    GoCardless,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct StartLinkingProcess {
    pub external_bank_institution_id: Snowflake,
}

/// Starts the process to link a bank account to financrr.
#[utoipa::path(post,
    path = "/start-linking-process",
    context_path = "/api/v1/bank-account-linking",
    tag = "Bank Account Linking",
    responses(
        (status = StatusCode::CREATED, description = "Successfully started the bank account linking process.", content_type="application/json", body = StartLinkingProcessResponse),
        NotFoundResponse,
        GeneralInternalServerErrorResponse,
        UnauthorizedResponse
    ),
    security(
        ("bearer_token" = [])
    ),
)]
#[axum::debug_handler]
async fn start_linking_process(
    State(ctx): State<AppContext>,
    _: Authenticated<sessions::Model>,
    Extension(bank_linking_data): Extension<BankLinkingData>,
    Extension(snowflake_generator): Extension<SnowflakeGenerator>,
    Json(data): Json<StartLinkingProcess>,
) -> AppResult<(StatusCode, Json<StartLinkingProcessResponse>)> {
    let institution = ExternalBankInstitutions::find_by_id(&ctx.db, data.external_bank_institution_id.id)
        .await?
        .ok_or(AppError::NotFound())?;

    if let Some(go_cardless_client) = bank_linking_data.get_go_cardless_client() {
        if Model::find_by_external_institution_id(&ctx.db, institution.id)
            .await?
            .is_none()
        {
            let agreement = go_cardless_client
                .create_end_user_agreement(
                    institution.external_id.as_str(),
                    institution
                        .access_valid_for_days
                        .map(|i| i as u16)
                        .unwrap_or(DEFAULT_ACCESS_VALID_FOR),
                )
                .await?;

            _ = Entity::create_from_api_response(&ctx.db, &snowflake_generator, &agreement, institution.id).await?;
        }
    }

    Ok((
        StatusCode::CREATED,
        Json(StartLinkingProcessResponse {
            flow: LinkingFlows::GoCardless,
        }),
    ))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/bank-account-linking")
        .add("/start-linking-process", post(start_linking_process))
}
