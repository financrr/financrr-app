use croner::Cron;
use deschuler::cron_builder::CronBuilder;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use time::OffsetDateTime;
use utoipa::ToSchema;

use entity::utility::time::get_now;
use utility::datetime::{convert_chrono_to_time, convert_time_to_chrono};

use crate::api::error::api::ApiError;
use crate::util::cron::get_cron_builder_default;
use crate::wrapper::entity::transaction::recurring::recurring_rule::dto::{CronPatternDTO, RecurringRuleDTO};

pub(crate) mod dto;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub(crate) enum RecurringRule {
    #[serde(rename = "cron_pattern")]
    CronPattern(CronPattern),
    #[serde(rename = "special")]
    Special(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub(crate) struct CronPattern {
    pub(crate) second: String,
    pub(crate) minute: String,
    pub(crate) hour: String,
    pub(crate) day_of_month: String,
    pub(crate) month: String,
    pub(crate) day_of_week: String,
}

impl CronPattern {
    pub(crate) fn from_dto(dto: CronPatternDTO, now: OffsetDateTime) -> Self {
        Self {
            second: now.second().to_string(),
            minute: now.minute().to_string(),
            hour: now.hour().to_string(),
            day_of_month: dto.day_of_month,
            month: dto.month,
            day_of_week: dto.day_of_week,
        }
    }
}

impl RecurringRule {
    pub(crate) fn to_json_value(&self) -> Result<Value, ApiError> {
        serde_json::to_value(self).map_err(ApiError::from)
    }

    pub(crate) fn from_json_value(value: Value) -> Result<Self, ApiError> {
        serde_json::from_value(value).map_err(ApiError::from)
    }

    pub(crate) fn from_recurring_ruled_dto(dto: RecurringRuleDTO, now: OffsetDateTime) -> Self {
        match dto {
            RecurringRuleDTO::CronPattern(cron_dto) => Self::CronPattern(CronPattern::from_dto(cron_dto, now)),
            RecurringRuleDTO::Special(special) => Self::Special(special),
        }
    }

    pub(crate) fn to_cron(&self) -> Result<Cron, ApiError> {
        match self {
            Self::CronPattern(pattern) => Self::build_cron(pattern, get_cron_builder_default()),
            Self::Special(special) => Self::build_special(special),
        }
    }

    fn build_cron(pattern: &CronPattern, mut cron_builder: CronBuilder) -> Result<Cron, ApiError> {
        cron_builder.second(pattern.second.clone());
        cron_builder.minute(pattern.minute.clone());
        cron_builder.hour(pattern.hour.clone());

        cron_builder.day_of_month(pattern.day_of_month.clone());
        cron_builder.month(pattern.month.clone());
        cron_builder.day_of_week(pattern.day_of_week.clone());

        cron_builder.build().map_err(ApiError::from)
    }

    fn build_special(special: &str) -> Result<Cron, ApiError> {
        Cron::new(special).parse().map_err(ApiError::from)
    }

    pub(crate) fn find_next_occurrence(&self, now: &OffsetDateTime) -> Option<OffsetDateTime> {
        let now_chrono = convert_time_to_chrono(now).ok()?;

        let next_occurrence_chrono =
            self.to_cron().ok().and_then(|cron| cron.find_next_occurrence(&now_chrono, false).ok())?;

        convert_chrono_to_time(&next_occurrence_chrono).ok()
    }
}

impl From<RecurringRuleDTO> for RecurringRule {
    fn from(dto: RecurringRuleDTO) -> Self {
        Self::from_recurring_ruled_dto(dto, get_now())
    }
}
