use serde::Serialize;
use time::OffsetDateTime;
use utoipa::ToSchema;

use entity::utility::time::get_now;

#[derive(Serialize, ToSchema)]
pub(crate) struct HealthResponse {
    pub(crate) healthy: bool,
    pub(crate) api_version: u8,
    #[serde(with = "time::serde::rfc3339")]
    pub(crate) local_time_now: OffsetDateTime,
    pub(crate) details: Option<String>,
}

impl HealthResponse {
    pub(crate) fn new(healthy: bool, details: Option<String>) -> Self {
        Self {
            healthy,
            api_version: 1,
            local_time_now: get_now(),
            details,
        }
    }
}
