use crate::models::_entities::sessions;
use crate::models::_entities::sessions::Model;
use crate::models::users;
use crate::types::snowflake::Snowflake;
use crate::views::user::UserResponse;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SessionResponse {
    pub id: Snowflake,
    pub user: UserResponse,
    pub api_key: String,
    pub name: Option<String>,
    pub user_agent: Option<String>,
    pub last_accessed_at: Option<DateTime<FixedOffset>>,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

impl From<(sessions::Model, users::Model)> for SessionResponse {
    fn from(value: (Model, users::Model)) -> Self {
        let user = UserResponse::from(value.1);
        let session = value.0;
        Self {
            id: Snowflake::new(session.id),
            user,
            api_key: session.api_key,
            name: session.name,
            user_agent: session.user_agent,
            last_accessed_at: session.last_accessed_at,
            created_at: session.created_at,
            updated_at: session.updated_at,
        }
    }
}
