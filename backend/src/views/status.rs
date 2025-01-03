use crate::constants::{ApiVersions, ALL_API_VERSIONS, CURRENT_API_VERSION};
use serde::Serialize;
use std::sync::LazyLock;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct HealthResponse {
    pub status: HealthStatus,
    pub database_status: HealthReport,
    pub cache_status: HealthReport,
    pub storage_status: HealthReport,
}

#[derive(Debug, Clone, Copy, Serialize, ToSchema)]
pub enum HealthStatus {
    Healthy,
    Unhealthy,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct HealthReport {
    pub status: HealthStatus,
    pub failed_components: Option<Vec<StatusComponents>>,
}

#[derive(Debug, Clone, Copy, Serialize, ToSchema)]
pub enum StatusComponents {
    Database,
    CacheInsertion,
    CacheRetrieval,
    CacheDeletion,
    StorageInsertion,
    StorageRetrieval,
    StorageDeletion,
}

#[derive(Debug, Copy, Clone, Serialize, ToSchema)]
pub struct VersionResponse {
    api_version: &'static ApiVersions,
    all_api_versions: [&'static ApiVersions; 1],
}

impl Default for VersionResponse {
    fn default() -> Self {
        static INSTANCE: LazyLock<VersionResponse> = LazyLock::new(|| VersionResponse {
            api_version: &CURRENT_API_VERSION,
            all_api_versions: ALL_API_VERSIONS,
        });

        *INSTANCE
    }
}
