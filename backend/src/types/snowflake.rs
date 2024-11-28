use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::fmt::{Display, Formatter};
use utoipa::openapi::path::{Parameter, ParameterBuilder, ParameterIn};
use utoipa::openapi::schema::SchemaType;
use utoipa::openapi::{KnownFormat, ObjectBuilder, RefOr, Required, Schema, SchemaFormat, Type};
use utoipa::{IntoParams, PartialSchema};

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct Snowflake {
    pub id: i64,
}

impl Snowflake {
    pub fn new(id: i64) -> Self {
        Self { id }
    }
}

impl From<i64> for Snowflake {
    fn from(value: i64) -> Self {
        Self::new(value)
    }
}

impl From<Snowflake> for i64 {
    fn from(value: Snowflake) -> Self {
        Self::from(&value)
    }
}

impl From<&Snowflake> for i64 {
    fn from(value: &Snowflake) -> Self {
        value.id
    }
}

impl Display for Snowflake {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl PartialSchema for Snowflake {
    fn schema() -> RefOr<Schema> {
        ObjectBuilder::new()
            .description(Some("A unique identifier for an entity."))
            .schema_type(SchemaType::Type(Type::String))
            .format(Some(SchemaFormat::KnownFormat(KnownFormat::Int64)))
            .examples(vec![serde_json::json!("60503861139345408")])
            .minimum(Some(0f64))
            .build()
            .into()
    }
}

impl IntoParams for Snowflake {
    fn into_params(parameter_in_provider: impl Fn() -> Option<ParameterIn>) -> Vec<Parameter> {
        vec![ParameterBuilder::new()
            .name("Snowflake")
            .description(Some("The snowflake ID."))
            .required(Required::True)
            .parameter_in(parameter_in_provider().unwrap_or(ParameterIn::Path))
            .schema(Some(Self::schema()))
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
