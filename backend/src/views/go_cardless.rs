use crate::types::snowflake::Snowflake;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct CreateLinkingLinkResponse {
    pub id: Snowflake,
    pub link: String,
}
