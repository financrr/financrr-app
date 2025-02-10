use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

// TODO: remove workaround when https://github.com/loco-rs/loco/pull/984 is merged
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct Pager<T> {
    pub info: PagerMeta,
    pub results: Vec<T>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct PagerMeta {
    pub page: u64,
    pub page_size: u64,
    pub total_pages: u64,
    pub total: u64,
}

#[derive(Debug, Deserialize, Serialize, IntoParams)]
pub struct PaginationQuery {
    #[serde(default = "default_page_size", deserialize_with = "deserialize_pagination_filter")]
    pub page_size: u64,
    #[serde(default = "default_page", deserialize_with = "deserialize_pagination_filter")]
    pub page: u64,
}

impl From<PaginationQuery> for loco_rs::model::query::PaginationQuery {
    fn from(query: PaginationQuery) -> Self {
        Self {
            page: query.page,
            page_size: query.page_size,
        }
    }
}

fn deserialize_pagination_filter<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.parse().map_err(serde::de::Error::custom)
}

const fn default_page_size() -> u64 {
    25
}

const fn default_page() -> u64 {
    1
}
