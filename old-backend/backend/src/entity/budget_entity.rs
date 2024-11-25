use crate::snowflake::snowflake_type::Snowflake;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash, ToSchema, Serialize)]
pub(crate) struct Budget {
    pub(crate) id: Snowflake,
    pub(crate) user: Snowflake,
    pub(crate) amount: i64,
    pub(crate) name: String,
    pub(crate) description: Option<String>,
}
