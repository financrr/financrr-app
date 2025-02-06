use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct CreateLinkingLinkResponse {
    pub link: String,
}
