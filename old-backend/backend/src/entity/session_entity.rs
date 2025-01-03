use serde::Serialize;
use utoipa::ToSchema;
use crate::entity::db_model::session::Model;
use crate::snowflake::snowflake_type::Snowflake;

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash, ToSchema, Serialize)]
pub(crate) struct Session {
    pub(crate) id: Snowflake,
    pub(crate) token: String,
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) platform: Option<String>,
    pub(crate) user_id: Snowflake,
}

impl From<Model> for Session {
    fn from(value: Model) -> Self {
        Self {
            id: Snowflake::new(value.id),
            token: value.token,
            name: value.name,
            description: value.description,
            platform: value.platform,
            user_id: Snowflake::new(value.user),
        }
    }
}
