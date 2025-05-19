use crate::error::app_error::AppResult;
use crate::types::snowflake::Snowflake;
use axum::http::StatusCode;
use loco_rs::prelude::Routes;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub enum LinkingFlow {
    Oauth2(String),
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LinkBankAccount {
    pub external_bank_institution_id: Snowflake,
}

// #[utoipa::path(post,)]
async fn link_account() -> AppResult<(StatusCode, LinkingFlow)> {
    unimplemented!("Still work in progress")
}

pub fn routes() -> Routes {
    Routes::new().prefix("/bank-data-importer")
}
