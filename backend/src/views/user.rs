use crate::models::users::Model;
use crate::types::snowflake::Snowflake;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct RegistrationResponse {
    pub id: Snowflake,
    pub email: String,
    pub name: String,
    pub flags: i32,
    pub email_verification_sent_at: Option<DateTimeWithTimeZone>,
    pub email_verified_at: Option<DateTimeWithTimeZone>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

impl From<Model> for RegistrationResponse {
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
