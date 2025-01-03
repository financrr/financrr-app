use std::time::SystemTimeError;

use chrono::{DateTime, FixedOffset, MappedLocalTime};
use sea_orm::DbErr;
use serde::Serialize;
use thiserror::Error;
use time::error::ComponentRange;

#[derive(Debug, Clone, Error, Serialize)]
pub enum TimeError {
    #[error("Time component range error")]
    TimeComponentRange(
        #[from]
        #[serde(with = "crate::util::serde::time_error::component_range")]
        ComponentRange,
    ),
    #[error("Out of bounds error")]
    OutOfBounds,
    #[error("Mapped local time error")]
    MappedLocalTime(
        #[serde(with = "crate::util::serde::time_error::local_result")] MappedLocalTime<DateTime<FixedOffset>>,
    ),
    #[error("System time error")]
    SystemTimeError(
        #[from]
        #[serde(with = "crate::util::serde::time_error::system_time_error")]
        SystemTimeError,
    ),
}

impl TimeError {
    pub fn from_mapped_local_time(
        mapped: MappedLocalTime<DateTime<FixedOffset>>,
    ) -> Result<DateTime<FixedOffset>, Self> {
        match mapped {
            MappedLocalTime::None => Err(Self::OutOfBounds),
            MappedLocalTime::Single(datetime) => Ok(datetime),
            MappedLocalTime::Ambiguous(ear, lat) => Err(Self::MappedLocalTime(MappedLocalTime::Ambiguous(ear, lat))),
        }
    }

    pub fn into_db_err(self) -> DbErr {
        DbErr::Custom(self.to_string())
    }
}
