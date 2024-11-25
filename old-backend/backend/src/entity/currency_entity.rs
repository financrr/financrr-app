use serde::Serialize;
use utoipa::ToSchema;
use crate::entity::db_model::currency::Model;
use crate::snowflake::snowflake_type::Snowflake;

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash, ToSchema, Serialize)]
pub(crate) struct Currency {
    pub(crate) id: Snowflake,
    pub(crate) name: String,
    pub(crate) symbol: String,
    pub(crate) iso_code: Option<String>,
    pub(crate) decimal_places: u32,
    pub(crate) user: Option<Snowflake>,
}

impl From<Model> for Currency {
    fn from(value: Model) -> Self {
        Self {
            id: Snowflake::from(value.id),
            name: value.name,
            symbol: value.symbol,
            iso_code: value.iso_code,
            decimal_places: value.decimal_places as u32,
            user: value.user.map(Snowflake::from),
        }
    }
}
