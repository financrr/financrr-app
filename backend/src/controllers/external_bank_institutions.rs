use crate::error::app_error::AppResult;
use crate::error::app_error::GeneralInternalServerErrorResponse;
use crate::middlewares::authentication::Authenticated;
use crate::models::_entities::sessions;
use crate::opensearch::indices::OpensearchIndex;
use crate::opensearch::models::external_bank_institutions::IndexableExternalBankInstitution;
use crate::services::opensearch::client::OpensearchClient;
use crate::views::external_bank_institutions::{
    ExternalBankInstitutionCountryOverviewResponse, ExternalBankInstitutionResponse,
};
use crate::views::pagination::{Pager, PaginationQuery};
use axum::extract::Query;
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Extension, Json};
use loco_rs::prelude::Routes;
use serde::Deserialize;
use serde_json::{json, Value};
use std::sync::LazyLock;
use tracing::error;
use utoipa::IntoParams;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct SearchQuery {
    #[validate(length(min = 1))]
    #[serde(default)]
    pub fts: String,
    #[validate(length(min = 2, max = 3))]
    #[serde(default)]
    pub country: String,
}

/// Search for supported bank institutions
#[utoipa::path(get,
    path = "/",
    context_path = "/api/v1/external-bank-institutions",
    tag = "External Bank Institutions",
    responses(
        (status = StatusCode::OK, description = "Successfully retrieved External Bank Institutions.", content_type="application/json", body = Pager<ExternalBankInstitutionResponse>),
        GeneralInternalServerErrorResponse,
    ),
    params(
        SearchQuery,
        PaginationQuery,
    ),
    security(
        ("bearer_token" = [])
    ),
)]
async fn get_external_bank_institutions(
    Extension(opensearch): Extension<OpensearchClient>,
    Query(search_query): Query<SearchQuery>,
    Query(pagination): Query<PaginationQuery>,
    _: Authenticated<sessions::Model>,
) -> AppResult<(StatusCode, Json<Pager<ExternalBankInstitutionResponse>>)> {
    let query = build_search_query(&search_query, &pagination);

    let pager = opensearch
        .search::<IndexableExternalBankInstitution>(
            OpensearchIndex::EXTERNAL_BANK_INSTITUTIONS.name,
            query,
            pagination.page,
            pagination.page_size,
        )
        .await?;

    let mapped_pager = Pager {
        results: pager
            .results
            .into_iter()
            .map(ExternalBankInstitutionResponse::from)
            .collect(),
        info: pager.info,
    };

    Ok((StatusCode::OK, Json(mapped_pager)))
}

/// Get countries overview
///
/// A faceted list of all countries that we support plus the number of banks we support in that
/// specific country.
#[utoipa::path(get,
    path = "/countries-overview",
    context_path = "/api/v1/external-bank-institutions",
    tag = "External Bank Institutions",
    responses(
        (status = StatusCode::OK, description = "Successfully retrieved External Bank Institutions.", content_type="application/json", body = Vec<ExternalBankInstitutionCountryOverviewResponse>),
        GeneralInternalServerErrorResponse,
    ),
    security(
        ("bearer_token" = [])
    ),
)]
async fn get_countries_overview(
    Extension(opensearch): Extension<OpensearchClient>,
    _: Authenticated<sessions::Model>,
) -> AppResult<(StatusCode, Json<Vec<ExternalBankInstitutionCountryOverviewResponse>>)> {
    static BODY: LazyLock<Value> = LazyLock::new(|| {
        json!(
            {
              "size": 0,
              "aggs": {
                "countries_facets": {
                  "terms": {
                    "field": "countries",
                    "size": 10000
                  }
                }
              }
            }
        )
    });

    let response = opensearch
        .search_custom(OpensearchIndex::EXTERNAL_BANK_INSTITUTIONS.name, BODY.clone())
        .await?;

    let default_buckets = vec![];
    let buckets = response["aggregations"]["countries_facets"]["buckets"]
        .as_array()
        .unwrap_or_else(|| {
            error!(
                "Could not extract aggregations data from opensearch. Response: {}",
                response.to_string()
            );

            &default_buckets
        });

    let result: Vec<ExternalBankInstitutionCountryOverviewResponse> = buckets
        .iter()
        .map(|bucket| ExternalBankInstitutionCountryOverviewResponse {
            country: bucket["key"].as_str().unwrap_or_default().to_string(),
            count: bucket["doc_count"].as_u64().unwrap_or(0),
        })
        .collect();

    Ok((StatusCode::OK, Json(result)))
}

fn build_search_query(search_query: &SearchQuery, pagination: &PaginationQuery) -> Value {
    let fts = search_query.fts.trim();
    let country = search_query.country.trim();

    let mut query = json!({
        "from": (pagination.page - 1) * pagination.page_size,
        "size": pagination.page_size,
        "query": {
            "bool": {
                "must": []
            }
        }
    });

    if !fts.is_empty() {
        query["query"]["bool"]["must"].as_array_mut().unwrap().push(json!({
            "multi_match": {
                "query": fts,
                "fields": ["name", "bic"]
            }
        }));
    }

    if !country.is_empty() {
        query["query"]["bool"]["must"].as_array_mut().unwrap().push(json!({
            "term": {
                "countries": country
            }
        }));
    }

    query
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/external-bank-institutions")
        .add("/", get(get_external_bank_institutions))
        .add("/countries-overview", get(get_countries_overview))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_build_search_query_with_fts_and_country() {
        let search_query = SearchQuery {
            fts: "test".to_string(),
            country: "US".to_string(),
        };
        let pagination = PaginationQuery { page: 1, page_size: 10 };

        let expected_query = json!({
            "from": 0,
            "size": 10,
            "query": {
                "bool": {
                    "must": [
                        {
                            "multi_match": {
                                "query": "test",
                                "fields": ["name", "bic"]
                            }
                        },
                        {
                            "term": {
                                "countries": "US"
                            }
                        }
                    ]
                }
            }
        });

        let actual_query = build_search_query(&search_query, &pagination);
        assert_eq!(expected_query, actual_query);
    }

    #[test]
    fn test_build_search_query_with_fts_only() {
        let search_query = SearchQuery {
            fts: "test".to_string(),
            country: "".to_string(),
        };
        let pagination = PaginationQuery { page: 1, page_size: 10 };

        let expected_query = json!({
            "from": 0,
            "size": 10,
            "query": {
                "bool": {
                    "must": [
                        {
                            "multi_match": {
                                "query": "test",
                                "fields": ["name", "bic"]
                            }
                        }
                    ]
                }
            }
        });

        let actual_query = build_search_query(&search_query, &pagination);
        assert_eq!(expected_query, actual_query);
    }

    #[test]
    fn test_build_search_query_with_country_only() {
        let search_query = SearchQuery {
            fts: "".to_string(),
            country: "US".to_string(),
        };
        let pagination = PaginationQuery { page: 1, page_size: 10 };

        let expected_query = json!({
            "from": 0,
            "size": 10,
            "query": {
                "bool": {
                    "must": [
                        {
                            "term": {
                                "countries": "US"
                            }
                        }
                    ]
                }
            }
        });

        let actual_query = build_search_query(&search_query, &pagination);
        assert_eq!(expected_query, actual_query);
    }

    #[test]
    fn test_build_search_query_with_no_fts_and_no_country() {
        let search_query = SearchQuery {
            fts: "".to_string(),
            country: "".to_string(),
        };
        let pagination = PaginationQuery { page: 1, page_size: 10 };

        let expected_query = json!({
            "from": 0,
            "size": 10,
            "query": {
                "bool": {
                    "must": []
                }
            }
        });

        let actual_query = build_search_query(&search_query, &pagination);
        assert_eq!(expected_query, actual_query);
    }

    #[test]
    fn test_build_search_query_with_only_spaces() {
        let search_query = SearchQuery {
            fts: " ".to_string(),
            country: "     ".to_string(),
        };
        let pagination = PaginationQuery { page: 1, page_size: 10 };

        let expected_query = json!({
            "from": 0,
            "size": 10,
            "query": {
                "bool": {
                    "must": []
                }
            }
        });

        let actual_query = build_search_query(&search_query, &pagination);
        assert_eq!(expected_query, actual_query);
    }
}
