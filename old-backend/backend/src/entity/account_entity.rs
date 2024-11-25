use serde::Serialize;
use utoipa::ToSchema;
use crate::snowflake::snowflake_type::Snowflake;

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash, ToSchema, Serialize)]
pub(crate) struct Account {
    pub(crate) id: Snowflake,
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) iban: Option<String>,
    pub(crate) balance: i64,
    pub(crate) original_balance: i64,
    pub(crate) currency: Snowflake,
}
