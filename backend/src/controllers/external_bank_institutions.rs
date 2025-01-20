use crate::error::app_error::AppResult;
use crate::error::app_error::GeneralInternalServerErrorResponse;
use crate::middlewares::authentication::Authenticated;
use crate::models::_entities::external_bank_institutions::Column;
use crate::models::_entities::sessions;
use crate::models::external_bank_institutions::Entity;
use crate::views::external_bank_institutions::ExternalBankInstitutionResponse;
use crate::views::pagination::{Pager, PaginationQuery};
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::routing::get;
use axum::{debug_handler, Json};
use loco_rs::app::AppContext;
use loco_rs::model::query;
use loco_rs::prelude::Routes;
use sea_orm::EntityTrait;
use serde::Deserialize;
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
    context_path = "/api/v1/external_bank_institutions",
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

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/external_bank_institutions")
        .add("/", get(get_external_bank_institutions))
}
