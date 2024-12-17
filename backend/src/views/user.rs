use crate::models::users::Model;
use crate::types::snowflake::Snowflake;
use chrono::{DateTime, FixedOffset};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct UserResponse {
    pub id: Snowflake,
    pub email: String,
    pub name: String,
    pub flags: i32,
    pub email_verification_sent_at: Option<DateTime<FixedOffset>>,
    pub email_verified_at: Option<DateTime<FixedOffset>>,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

impl From<Model> for UserResponse {
    fn from(value: Model) -> Self {
        Self {
            id: Snowflake::new(value.id),
            email: value.email,
            name: value.name,
            flags: value.flags,
            email_verification_sent_at: value.email_verification_sent_at,
            email_verified_at: value.email_verified_at,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
