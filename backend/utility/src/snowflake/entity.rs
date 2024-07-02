use std::fmt;
use std::fmt::{Display, Formatter};

use sea_orm::Value;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::json;
use utoipa::openapi::path::{Parameter, ParameterBuilder, ParameterIn};
use utoipa::openapi::{KnownFormat, ObjectBuilder, RefOr, Required, Schema, SchemaFormat, SchemaType};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct Snowflake {
    pub id: i64,
}

impl Snowflake {
    pub fn new(id: i64) -> Self {
        Self {
            id,
        }
    }
}

impl From<i64> for Snowflake {
    fn from(value: i64) -> Self {
        Self::new(value)
    }
}

impl From<Snowflake> for i64 {
    fn from(value: Snowflake) -> Self {
        value.id
    }
}

impl From<Snowflake> for Value {
    fn from(value: Snowflake) -> Self {
        Self::BigInt(Some(value.id))
    }
}

impl Display for Snowflake {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl ToSchema<'static> for Snowflake {
    fn schema() -> (&'static str, RefOr<Schema>) {
        (
            "Snowflake",
            ObjectBuilder::new()
                .schema_type(SchemaType::String)
                .format(Some(SchemaFormat::KnownFormat(KnownFormat::Int64)))
                .example(Some(json!("60503861139345408")))
                .minimum(Some(0f64))
                .nullable(false)
                .build()
                .into(),
        )
    }
}

impl IntoParams for Snowflake {
    fn into_params(parameter_in_provider: impl Fn() -> Option<ParameterIn>) -> Vec<Parameter> {
        vec![ParameterBuilder::new()
            .name("Snowflake")
            .description(Some("The snowflake ID."))
            .required(Required::True)
            .parameter_in(parameter_in_provider().unwrap_or(ParameterIn::Path))
            .schema(Some(Self::schema().1))
            .build()]
    }
}

impl Serialize for Snowflake {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.id.to_string())
    }
}

impl<'de> Deserialize<'de> for Snowflake {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse::<i64>().map_err(Error::custom).map(Self::new)
    }
}
