use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use utoipa::ToSchema;

use crate::wrapper::entity::session::Session;
use crate::wrapper::entity::user::User;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub(crate) struct PublicSession {
    pub(crate) id: i64,
    pub(crate) user_id: i64,
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) platform: Option<String>,
    #[serde(with = "time::serde::rfc3339")]
    pub(crate) expires_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub(crate) created_at: OffsetDateTime,
    pub(crate) user: User,
}

impl From<Session> for PublicSession {
    fn from(value: Session) -> Self {
        Self {
            id: value.snowflake.id,
            user_id: value.user.snowflake.id,
            name: value.name,
            description: value.description,
            platform: value.platform,
            expires_at: value.expires_at,
            created_at: value.created_at,
            user: value.user,
        }
    }
}
