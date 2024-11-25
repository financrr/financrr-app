use actix_web::http::Uri;
use actix_web::web::{Json, Path};
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use actix_web_validation::Validated;
use utility::snowflake::entity::Snowflake;

use crate::api::documentation::response::{InternalServerError, ResourceNotFound, Unauthorized, ValidationError};
use crate::api::error::api::ApiError;
use crate::api::pagination::{PageSizeParam, Pagination};
use crate::wrapper::entity::currency::dto::CurrencyDTO;
use crate::wrapper::entity::currency::Currency;
use crate::wrapper::entity::user::User;
use crate::wrapper::permission::{HasPermissionOrError, Permissions};
use crate::wrapper::types::phantom::{Identifiable, Phantom};

pub(crate) fn currency_controller(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/currency")
            .service(get_all_currencies)
            .service(create_currency)
            .service(delete_currency)
            .service(update_currency)
            .service(get_one_currency),
    );
}

#[utoipa::path(get,
    responses(
        (status = 200, description = "Successfully retrieved all Currencies.", content_type = "application/json", body = PaginatedCurrency),
        ValidationError,
        Unauthorized,
        InternalServerError,
    ),
    security(
        ("bearer_token" = [])
    ),
    params(PageSizeParam),
    path = "/api/v1/currency",
    tag = "Currency")]
#[get("")]
pub(crate) async fn get_all_currencies(
    user: Option<Phantom<User>>,
    page_size: PageSizeParam,
    uri: Uri,
) -> Result<impl Responder, ApiError> {
    if let Some(user) = user {
        let size = Currency::count_all_with_no_user_and_user(user.get_id()).await?;
        let currencies = Currency::find_all_with_no_user_and_user_paginated(user.get_id(), &page_size).await?;

        Ok(HttpResponse::Ok().json(Pagination::new(currencies, &page_size, size, uri)))
    } else {
        let size = Currency::count_all_with_no_user().await?;
        let currencies = Currency::find_all_with_no_user_paginated(&page_size).await?;

        Ok(HttpResponse::Ok().json(Pagination::new(currencies, &page_size, size, uri)))
    }
}

#[utoipa::path(get,
    responses(
        (status = 200, description = "Successfully retrieved the Currency.", content_type = "application/json", body = Currency),
        Unauthorized,
        ResourceNotFound,
        InternalServerError,
    ),
    security(
        ("bearer_token" = [])
    ),
    params(("currency_id" = Snowflake,)),
    path = "/api/v1/currency/{currency_id}",
    tag = "Currency")]
#[get("/{currency_id}")]
pub(crate) async fn get_one_currency(
    user: Option<Phantom<User>>,
    currency_id: Path<Snowflake>,
) -> Result<impl Responder, ApiError> {
    let currency_id = currency_id.into_inner();
    let user_id = user.map_or(Snowflake::from(-1), |user| user.get_id());

    Ok(HttpResponse::Ok().json(Currency::find_by_id_include_user(currency_id, user_id).await?))
}

#[utoipa::path(post,
responses(
(status = 201, description = "Successfully created the Currency.", content_type = "application/json", body = Currency),
Unauthorized,
InternalServerError,
),
security(
("bearer_token" = [])
),
path = "/api/v1/currency",
request_body = CurrencyDTO,
tag = "Currency")]
#[post("")]
pub(crate) async fn create_currency(
    user: Phantom<User>,
    currency: Validated<Json<CurrencyDTO>>,
) -> Result<impl Responder, ApiError> {
    Ok(HttpResponse::Created().json(Currency::new(currency.into_inner().into_inner(), user.get_id()).await?))
}

#[utoipa::path(delete,
    responses(
        (status = 200, description = "Successfully deleted the Currency."),
        Unauthorized,
        ResourceNotFound,
        InternalServerError,
    ),
    security(
        ("bearer_token" = [])
    ),
    params(("currency_id" = Snowflake,)),
    path = "/api/v1/currency/{currency_id}",
    tag = "Currency")]
#[delete("/{currency_id}")]
pub(crate) async fn delete_currency(
    user: Phantom<User>,
    currency_id: Path<Snowflake>,
) -> Result<impl Responder, ApiError> {
    let currency = Currency::find_by_id(currency_id.into_inner()).await?;
    currency.has_permission_or_error(user.get_id(), Permissions::READ_DELETE).await?;

    currency.delete().await?;
    Ok(HttpResponse::NoContent())
}

#[utoipa::path(patch,
responses(
(status = 200, description = "Successfully updated the Currency.", content_type = "application/json", body = Currency),
Unauthorized,
ResourceNotFound,
InternalServerError,
),
security(
("bearer_token" = [])
),
params(("currency_id" = Snowflake,)),
path = "/api/v1/currency/{currency_id}",
request_body = CurrencyDTO,
tag = "Currency")]
#[patch("/{currency_id}")]
pub(crate) async fn update_currency(
    user: Phantom<User>,
    update_currency: CurrencyDTO,
    currency_id: Path<Snowflake>,
) -> Result<impl Responder, ApiError> {
    let currency = Currency::find_by_id(currency_id.into_inner()).await?;
    currency.has_permission_or_error(user.get_id(), Permissions::READ_WRITE).await?;

    Ok(HttpResponse::Ok().json(currency.update(update_currency).await?))
}
