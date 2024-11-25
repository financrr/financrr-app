use crate::api::error::api::ApiError;
use crate::entity::db_model::user::Model;
use crate::snowflake::snowflake_type::Snowflake;
use bitflags::bitflags;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash, ToSchema, Serialize)]
pub(crate) struct User {
    pub(crate) snowflake: Snowflake,
    pub(crate) username: String,
    pub(crate) email: Option<String>,
    pub(crate) display_name: Option<String>,
    #[serde(skip)]
    pub(crate) password: String,
    pub(crate) permissions: UserPermissions,
}

impl User {
    pub(crate) fn verify_password(&self, password: &str) -> Result<bool, ApiError> {
        argon2::verify_encoded(&self.password, password.as_bytes()).map_err(ApiError::from)
    }
}

impl From<Model> for User {
    fn from(value: Model) -> Self {
        Self {
            snowflake: Snowflake::from(value.id),
            username: value.username,
            email: value.email,
            display_name: value.display_name,
            password: value.password,
            permissions: UserPermissions::from_bits_truncate(value.permissions),
        }
    }
}

bitflags! {
    #[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash, ToSchema, Serialize)]
    pub(crate) struct UserPermissions: i32 {
        const USER = 0b001;
        const ADMIN = 0b010;
    }
}
