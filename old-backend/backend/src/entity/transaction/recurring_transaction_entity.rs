use crate::api::error::api::ApiError;
use crate::entity::db_model::recurring_transaction::Model;
use crate::entity::transaction::recurring_rule::RecurringRule;
use crate::snowflake::snowflake_type::Snowflake;
use chrono::{DateTime, FixedOffset};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash, ToSchema, Serialize)]
pub(crate) struct RecurringTransaction {
    pub(crate) id: Snowflake,
    pub(crate) template: Snowflake,
    pub(crate) recurring_rule: RecurringRule,
    pub(crate) last_executed_at: Option<DateTime<FixedOffset>>,
}

impl TryFrom<Model> for RecurringTransaction {
    type Error = ApiError;

    fn try_from(model: Model) -> Result<Self, Self::Error> {
        Ok(RecurringTransaction {
            id: Snowflake::new(model.id),
            template: Snowflake::new(model.template),
            recurring_rule: RecurringRule::try_from(model.recurring_rule)?,
            last_executed_at: model.last_executed_at,
        })
    }
}
