use crate::snowflake::snowflake_generator::{SNOWFLAKE_EPOCH, TIMESTAMP_SHIFT};
use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, TimeZone, Utc};
use derive_more::Display;
use sea_orm::sea_query::{ArrayType, ValueType, ValueTypeErr};
use sea_orm::{ColIdx, ColumnType, DeriveValueType, QueryResult, TryGetError, TryGetable, Value};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::ops::Add;
use std::time::{Duration, SystemTime};
use utoipa::openapi::path::{Parameter, ParameterBuilder, ParameterIn};
use utoipa::openapi::{KnownFormat, ObjectBuilder, RefOr, Required, Schema, SchemaFormat, SchemaType};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Display, Serialize, ToSchema)]
#[display("Snowflake({}, {})", id, created_at)]
pub(crate) struct Snowflake {
    pub(crate) id: i64,
    #[serde(skip_deserializing)]
    pub(crate) created_at: DateTime<Utc>,
}

impl Snowflake {
    pub(crate) fn new(value: i64) -> Self {
        Self {
            id: value,
            created_at: Self::calculate_created_at(value),
        }
    }

    fn calculate_created_at(id: i64) -> DateTime<Utc> {
        let timestamp = (id >> TIMESTAMP_SHIFT) + SNOWFLAKE_EPOCH;

        Utc.timestamp_millis_opt(timestamp).unwrap()
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

/* Serde */

impl Deserialize<'_> for Snowflake {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'_>,
    {
        let id = i64::deserialize(deserializer)?;

        Ok(Self::new(id))
    }
}

/* SeaOrm */

impl From<Snowflake> for Value {
    fn from(value: Snowflake) -> Self {
        Value::BigInt(Some(value.id))
    }
}

impl TryGetable for Snowflake {
    fn try_get_by<I: ColIdx>(res: &QueryResult, index: I) -> Result<Self, TryGetError> {
        let id = i64::try_get_by(res, index)?;

        Ok(Self::new(id))
    }
}

impl ValueType for Snowflake {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        i64::try_from(v).map(Self::new)
    }

    fn type_name() -> String {
        "Snowflake".to_owned()
    }

    fn array_type() -> ArrayType {
        ArrayType::BigInt
    }

    fn column_type() -> ColumnType {
        ColumnType::BigInteger
    }
}

/* Utoipa */

impl IntoParams for Snowflake {
    fn into_params(parameter_in_provider: impl Fn() -> Option<ParameterIn>) -> Vec<Parameter> {
        vec![
            ParameterBuilder::new()
                .name("Snowflake")
                .description(Some("Snowflake Id"))
                .required(Required::True)
                .parameter_in(parameter_in_provider().unwrap_or(ParameterIn::Path))
                .schema(Some(
                    ObjectBuilder::new()
                        .schema_type(SchemaType::String)
                        .format(Some(SchemaFormat::KnownFormat(KnownFormat::Int64)))
                        .example(Some(json!("60503861139345408")))
                        .minimum(Some(0f64))
                        .nullable(false)
                        .build()
                ))
                .build()
        ]
    }
}
