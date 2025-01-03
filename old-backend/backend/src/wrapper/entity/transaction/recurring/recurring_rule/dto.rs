use crate::api::error::validation::ValidationCode;
use const_format::concatcp;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::{Validate, ValidationErrors};

pub(crate) const SPECIALS: [&str; 5] = ["@yearly", "@annually", "@monthly", "@weekly", "@daily"];
pub(crate) const SPECIALS_STR: &str =
    concatcp!(SPECIALS[0], ", ", SPECIALS[1], ", ", SPECIALS[2], ", ", SPECIALS[3], ", ", SPECIALS[4]);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub(crate) enum RecurringRuleDTO {
    #[serde(rename = "cron_pattern")]
    CronPattern(CronPatternDTO),
    #[serde(rename = "special")]
    Special(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub(crate) struct CronPatternDTO {
    pub(crate) day_of_month: String,
    pub(crate) month: String,
    pub(crate) day_of_week: String,
}

//TODO replace with struct level validation
impl Validate for RecurringRuleDTO {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();
        match self {
            Self::CronPattern(inner) => {
                if inner.day_of_month.eq("*") && inner.month.eq("*") && inner.day_of_week.eq("*") {
                    errors.add("inner", ValidationCode::INVALID_CRON_PATTERN.into());
                }
            }
            Self::Special(special) => {
                if SPECIALS.iter().all(|&s| s != special) {
                    errors.add("special", ValidationCode::INVALID_SPECIAL_FIELD.into());
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
