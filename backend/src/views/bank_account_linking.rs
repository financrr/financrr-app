use crate::controllers::bank_account_linking::LinkingFlows;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct StartLinkingProcessResponse {
    pub flow: LinkingFlows,
}
