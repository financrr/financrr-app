use std::collections::HashMap;
use std::future::Future;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::api::error::api::ApiError;
use crate::api::pagination::PageSizeParam;

pub(crate) trait Searchable {
    fn create_index() -> impl Future<Output=()>;
    fn index() -> impl Future<Output=Result<(), ApiError>>;
    fn search(query: SearchQuery, user_id: i32, page_size: PageSizeParam) -> impl Future<Output=Result<Vec<Self>, ApiError>>
        where Self: Sized;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub(crate) struct SearchQuery {
    pub(crate) should: HashMap<String, String>,
    pub(crate) should_not: HashMap<String, String>,
    pub(crate) sort_by: HashMap<String, SortOrder>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub(crate) enum SortOrder {
    Asc,
    Desc,
}
