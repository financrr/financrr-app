use std::time::SystemTime;

use chrono::{DateTime, FixedOffset, Local, Offset, TimeZone};
use time::{OffsetDateTime, UtcOffset};

use crate::datetime::error::TimeError;

pub mod error;

pub fn get_now() -> Result<OffsetDateTime, TimeError> {
    Ok(OffsetDateTime::now_local().unwrap_or(get_now_from_chrono()?))
}

pub fn get_now_timestamp_millis() -> u64 {
    Local::now().timestamp_millis() as u64
}

pub fn get_epoch_millis() -> Result<u64, TimeError> {
    Ok(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_millis() as u64)
}

fn get_now_from_chrono() -> Result<OffsetDateTime, TimeError> {
    let local = Local::now();
    convert_chrono_to_time(&local.with_timezone(local.offset()))
}

pub fn get_utc_offset() -> Result<UtcOffset, TimeError> {
    let offset_seconds = Local::now().offset().fix().local_minus_utc();

    Ok(UtcOffset::from_whole_seconds(offset_seconds)?)
}

pub fn convert_chrono_to_time(chrono: &DateTime<FixedOffset>) -> Result<OffsetDateTime, TimeError> {
    let timestamp = chrono.timestamp();
    let offset_seconds = chrono.offset().fix().local_minus_utc();

    let utc_offset = UtcOffset::from_whole_seconds(offset_seconds)?;

    Ok(OffsetDateTime::from_unix_timestamp(timestamp)?.to_offset(utc_offset))
}

pub fn convert_time_to_chrono(time: &OffsetDateTime) -> Result<DateTime<FixedOffset>, TimeError> {
    let timestamp = time.unix_timestamp();
    let offset_seconds = time.offset().whole_seconds();
    let offset = FixedOffset::east_opt(offset_seconds).ok_or(TimeError::OutOfBounds)?;

    TimeError::from_mapped_local_time(offset.timestamp_opt(timestamp, 0))
}

pub fn extract_to_chrono_tz(datetime: &OffsetDateTime) -> Result<(FixedOffset, bool), TimeError> {
    let offset_seconds = datetime.offset().whole_seconds();
    let offset = FixedOffset::east_opt(offset_seconds).ok_or(TimeError::OutOfBounds)?;
    let fixed_offset = TimeZone::from_offset(&offset);

    let is_utc = offset_seconds == 0;

    Ok((fixed_offset, is_utc))
}

#[cfg(test)]
#[allow(clippy::all)]
mod tests {
    use chrono::prelude::*;

    use super::*;

    #[test]
    fn test_get_now() {
        let now = get_now().unwrap();
        // Check that the returned OffsetDateTime is not more than a few seconds away from the current time
        assert!((OffsetDateTime::now_utc() - now).whole_seconds().abs() < 5);
    }

    #[test]
    fn test_convert_chrono_to_time() {
        let chrono_now = Utc::now();
        let time_now = convert_chrono_to_time(&chrono_now.fixed_offset()).unwrap();
        // Check that the converted OffsetDateTime is not more than a few seconds away from the original DateTime
        assert!((chrono_now.timestamp() - time_now.unix_timestamp()).abs() < 5);
    }

    #[test]
    fn test_convert_time_to_chrono() {
        let time_now = OffsetDateTime::now_utc();
        let chrono_now = convert_time_to_chrono(&time_now).unwrap();
        // Check that the converted DateTime is not more than a few seconds away from the original OffsetDateTime
        assert!((time_now.unix_timestamp() - chrono_now.timestamp()).abs() < 5);
    }

    #[test]
    fn test_extract_to_chrono_tz() {
        let time_now = OffsetDateTime::now_utc();
        let (fixed_offset, is_utc) = extract_to_chrono_tz(&time_now).unwrap();
        // Check that the extracted timezone is UTC
        assert_eq!(fixed_offset, FixedOffset::east_opt(0).expect("Failed to create FixedOffset from 0 seconds"));
        assert!(is_utc);
    }
}
