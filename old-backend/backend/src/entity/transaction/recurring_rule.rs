use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;
use crate::api::error::api::ApiError;

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash, ToSchema, Serialize, Deserialize)]
pub(crate) enum RecurringRule {
    #[serde(rename = "cron_pattern")]
    CronPattern(CronPattern),
    #[serde(rename = "special")]
    Special(String),
}

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash, ToSchema, Serialize, Deserialize)]
pub(crate) struct CronPattern {
    pub(crate) second: String,
    pub(crate) minute: String,
    pub(crate) hour: String,
    pub(crate) day_of_month: String,
    pub(crate) month: String,
    pub(crate) day_of_week: String,
}

impl TryFrom<Value> for RecurringRule {
    type Error = ApiError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value).map_err(|_| ApiError::InvalidRecurringRule())
    }
}
