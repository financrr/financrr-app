use actix_web::http::Uri;
use actix_web::web::{Json, Path};
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use actix_web_validation::Validated;

use utility::snowflake::entity::Snowflake;

use crate::api::documentation::response::{InternalServerError, ResourceNotFound, Unauthorized, ValidationError};
use crate::api::error::api::ApiError;
use crate::api::pagination::{PageSizeParam, Pagination};
use crate::wrapper::entity::budget::dto::BudgetDTO;
use crate::wrapper::entity::budget::Budget;
use crate::wrapper::entity::user::User;
use crate::wrapper::permission::{HasPermissionByIdOrError, HasPermissionOrError, Permissions};
use crate::wrapper::types::phantom::{Identifiable, Phantom};

pub(crate) fn budget_controller(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/budget")
            .service(get_all_budgets)
            .service(get_transactions_from_budget)
            .service(create_budget)
            .service(delete_budget)
            .service(update_budget)
            .service(get_one_budget),
    );
}

#[utoipa::path(get,
    responses(
        (status = 200, description = "Successfully retrieved the Budgets.", content_type = "application/json", body = PaginatedBudget),
        ValidationError,
        Unauthorized,
        InternalServerError,
    ),
    params(PageSizeParam),
    security(
        ("bearer_token" = [])
    ),
    path = "/api/v1/budget",
    tag = "Budget"
)]
#[get("")]
pub(crate) async fn get_all_budgets(
    user: Phantom<User>,
    page_size: PageSizeParam,
    uri: Uri,
) -> Result<impl Responder, ApiError> {
    let total = Budget::count_all_by_user(user.get_id()).await?;
    let budgets = Budget::find_all_by_user_paginated(user.get_id(), &page_size).await?;

    Ok(HttpResponse::Ok().json(Pagination::new(budgets, &page_size, total, uri)))
}

#[utoipa::path(get,
    responses(
        (status = 200, description = "Successfully retrieved the Budget.", content_type = "application/json", body = Budget),
        Unauthorized,
        ResourceNotFound,
        InternalServerError,
    ),
    security(
        ("bearer_token" = [])
    ),
    params(("budget_id" = Snowflake,)),
    path = "/api/v1/budget/{budget_id}",
    tag = "Budget"
)]
#[get("/{budget_id}")]
pub(crate) async fn get_one_budget(
    user: Phantom<User>,
    budget_id: Path<Snowflake>,
) -> Result<impl Responder, ApiError> {
    let budget = Budget::find_by_id(budget_id.into_inner()).await?;
    budget.has_permission_or_error(user.get_id(), Permissions::READ).await?;

    Ok(HttpResponse::Ok().json(budget))
}

#[utoipa::path(get,
    responses(
        (status = 200, description = "Successfully retrieved the Transactions.", content_type = "application/json", body = PaginatedTransaction),
        Unauthorized,
        ResourceNotFound,
        InternalServerError,
    ),
    security(
        ("bearer_token" = [])
    ),
    params(("budget_id" = Snowflake,), PageSizeParam),
    path = "/api/v1/budget/{budget_id}/transactions",
    tag = "Budget"
)]
#[get("/{budget_id}/transactions")]
pub(crate) async fn get_transactions_from_budget(
    user: Phantom<User>,
    budget_id: Path<Snowflake>,
    page_size: PageSizeParam,
    uri: Uri,
) -> Result<impl Responder, ApiError> {
    let budget_id = budget_id.into_inner();
    Budget::has_permission_by_id_or_error(budget_id, user.get_id(), Permissions::READ).await?;

    let transactions = Budget::find_related_transactions_paginated(budget_id, &page_size).await?;
    let total = Budget::count_related_transactions(budget_id).await?;

    Ok(HttpResponse::Ok().json(Pagination::new(transactions, &page_size, total, uri)))
}

#[utoipa::path(post,
    responses(
        (status = 201, description = "Successfully created the Budget.", content_type = "application/json", body = Budget),
        ValidationError,
        Unauthorized,
        InternalServerError,
    ),
    security(
        ("bearer_token" = [])
    ),
    path = "/api/v1/budget",
    tag = "Budget"
)]
#[post("")]
pub(crate) async fn create_budget(
    user: Phantom<User>,
    budget: Validated<Json<BudgetDTO>>,
) -> Result<impl Responder, ApiError> {
    let budget = Budget::new(user.get_id(), budget.into_inner().into_inner()).await?;

    Ok(HttpResponse::Created().json(budget))
}

#[utoipa::path(delete,
    responses(
        (status = 204, description = "Successfully deleted the Budget."),
        Unauthorized,
        ResourceNotFound,
        InternalServerError,
    ),
    security(
        ("bearer_token" = [])
    ),
    params(("budget_id" = Snowflake,)),
    path = "/api/v1/budget/{budget_id}",
    tag = "Budget"
)]
#[delete("/{budget_id}")]
pub(crate) async fn delete_budget(user: Phantom<User>, budget_id: Path<Snowflake>) -> Result<impl Responder, ApiError> {
    let budget = Budget::find_by_id(budget_id.into_inner()).await?;
    budget.has_permission_or_error(user.get_id(), Permissions::READ_DELETE).await?;

    budget.delete().await?;

    Ok(HttpResponse::NoContent())
}

#[utoipa::path(patch,
    responses(
        (status = 200, description = "Successfully updated the Budget.", content_type = "application/json", body = Budget),
        ValidationError,
        Unauthorized,
        ResourceNotFound,
        InternalServerError,
    ),
    security(
        ("bearer_token" = [])
    ),
    params(("budget_id" = Snowflake,)),
    path = "/api/v1/budget/{budget_id}",
    tag = "Budget"
)]
#[patch("/{budget_id}")]
pub(crate) async fn update_budget(
    user: Phantom<User>,
    budget_id: Path<Snowflake>,
    budget_dto: Json<BudgetDTO>,
) -> Result<impl Responder, ApiError> {
    let budget = Budget::find_by_id(budget_id.into_inner()).await?;
    budget.has_permission_or_error(user.get_id(), Permissions::READ_WRITE).await?;

    let budget = budget.update(budget_dto.into_inner()).await?;

    Ok(HttpResponse::Ok().json(budget))
}
