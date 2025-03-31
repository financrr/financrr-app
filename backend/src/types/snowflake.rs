use migration::{ArrayType, ColumnType, Value, ValueTypeErr};
use sea_orm::sea_query::{Nullable, ValueType};
use sea_orm::{ColIdx, QueryResult, TryGetError, TryGetable, sea_query};
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::fmt::{Display, Formatter};
use utoipa::openapi::path::{Parameter, ParameterBuilder, ParameterIn};
use utoipa::openapi::schema::SchemaType;
use utoipa::openapi::{KnownFormat, ObjectBuilder, RefOr, Required, Schema, SchemaFormat, Type};
use utoipa::{IntoParams, PartialSchema, ToSchema};

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

impl ToSchema for Snowflake {}

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
        vec![
            ParameterBuilder::new()
                .name("Snowflake")
                .description(Some("The snowflake ID."))
                .required(Required::True)
                .parameter_in(parameter_in_provider().unwrap_or(ParameterIn::Path))
                .schema(Some(Self::schema()))
                .build(),
        ]
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

impl From<Snowflake> for sea_query::Value {
    fn from(value: Snowflake) -> Self {
        Self::from(value.id)
    }
}

impl TryGetable for Snowflake {
    fn try_get_by<I: ColIdx>(res: &QueryResult, index: I) -> Result<Self, TryGetError> {
        i64::try_get_by(res, index).map(Self::new)
    }
}

impl ValueType for Snowflake {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        <i64 as ValueType>::try_from(v).map(Self::new)
    }

    fn type_name() -> String {
        i64::type_name()
    }

    fn array_type() -> ArrayType {
        i64::array_type()
    }

    fn column_type() -> ColumnType {
        i64::column_type()
    }
}

impl Nullable for Snowflake {
    fn null() -> Value {
        i64::null()
    }
}
