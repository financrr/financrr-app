use crate::error::app_error::AppResult;
use crate::error::app_error::GeneralInternalServerErrorResponse;
use crate::middlewares::authentication::Authenticated;
use crate::models::_entities::external_bank_institutions::Column;
use crate::models::_entities::sessions;
use crate::models::external_bank_institutions::Entity;
use crate::opensearch::indices::OpensearchIndex;
use crate::services::opensearch::client::OpensearchClient;
use crate::views::external_bank_institutions::{
    ExternalBankInstitutionCountryOverviewResponse, ExternalBankInstitutionResponse,
};
use crate::views::pagination::{Pager, PaginationQuery};
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::routing::get;
use axum::{debug_handler, Extension, Json};
use loco_rs::app::AppContext;
use loco_rs::model::query;
use loco_rs::prelude::Routes;
use sea_orm::EntityTrait;
use serde::Deserialize;
use serde_json::{json, Value};
use std::sync::LazyLock;
use tracing::error;
use utoipa::IntoParams;

#[derive(Debug, Deserialize, IntoParams)]
pub struct SearchQuery {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub provider: String,
}

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
#[debug_handler]
async fn get_external_bank_institutions(
    State(ctx): State<AppContext>,
    Query(search_query): Query<SearchQuery>,
    Query(pagination): Query<PaginationQuery>,
    _: Authenticated<sessions::Model>,
) -> AppResult<(StatusCode, Json<Pager<ExternalBankInstitutionResponse>>)> {
    let mut condition = query::condition();
    let pagination = pagination.into();

    if !search_query.name.is_empty() {
        condition = condition.contains(Column::Name, search_query.name);
    }

    if !search_query.provider.is_empty() {
        condition = condition.eq(Column::Provider, search_query.provider);
    }

    let external_bank_institutions =
        query::paginate(&ctx.db, Entity::find(), Some(condition.build()), &pagination).await?;

    Ok((
        StatusCode::OK,
        Json(ExternalBankInstitutionResponse::response(
            external_bank_institutions,
            &pagination,
        )),
    ))
}

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
#[debug_handler]
async fn get_countries_overview(
    Extension(opensearch): Extension<OpensearchClient>,
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
        .search(OpensearchIndex::EXTERNAL_BANK_INSTITUTIONS.name, BODY.clone())
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

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/external-bank-institutions")
        .add("/", get(get_external_bank_institutions))
        .add("/countries-overview", get(get_countries_overview))
}
