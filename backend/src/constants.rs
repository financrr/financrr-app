use serde::Serialize;
use utoipa::ToSchema;

/// Api Versions
pub const CURRENT_API_VERSION: ApiVersions = ApiVersions::V1;
pub const ALL_API_VERSIONS: [&ApiVersions; 1] = [&ApiVersions::V1];

#[derive(Debug, Serialize, ToSchema)]
pub enum ApiVersions {
    V1,
}
