use actix_web::http::Uri;
use actix_web::web::{Path, ServiceConfig};
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};

use utility::snowflake::entity::Snowflake;

use crate::api::documentation::response::{InternalServerError, Unauthorized};
use crate::api::error::api::ApiError;
use crate::api::pagination::{PageSizeParam, Pagination};
use crate::wrapper::entity::transaction::recurring::dto::RecurringTransactionDTO;
use crate::wrapper::entity::transaction::recurring::RecurringTransaction;
use crate::wrapper::entity::user::User;
use crate::wrapper::permission::{HasPermissionOrError, Permissions};
use crate::wrapper::types::phantom::Phantom;

pub(crate) fn recurring_transaction_controller(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/recurring")
            .service(get_one_recurring_transaction)
            .service(get_all_recurring_transactions)
            .service(create_recurring_transaction)
            .service(delete_recurring_transaction)
            .service(update_recurring_transaction),
    );
}

#[utoipa::path(get,
    responses(
        (status = 200, description = "Successfully retrieved all Recurring Transactions.", content_type = "application/json", body = PaginatedRecurringTransaction),
        Unauthorized,
        InternalServerError,
    ),
    params(PageSizeParam),
    security(
        ("bearer_token" = [])
    ),
    path = "/api/v1/transaction/recurring",
    tag = "Recurring-Transaction")]
#[get("")]
pub(crate) async fn get_all_recurring_transactions(
    user: Phantom<User>,
    page_size: PageSizeParam,
    uri: Uri,
) -> Result<impl Responder, ApiError> {
    let total = RecurringTransaction::count_all_by_user_id(user.get_id()).await?;
    let transactions = RecurringTransaction::find_all_by_user_id_paginated(user.get_id(), &page_size).await?;

    Ok(HttpResponse::Ok().json(Pagination::new(transactions, &page_size, total, uri)))
}

#[utoipa::path(get,
    responses(
        (status = 200, description = "Successfully retrieved Recurring Transaction.", content_type = "application/json", body = RecurringTransaction),
        Unauthorized,
        InternalServerError,
    ),
    security(
        ("bearer_token" = [])
    ),
    params(("recurring_transaction_id" = Snowflake,)),
    path = "/api/v1/transaction/recurring/{recurring_transaction_id}",
    tag = "Recurring-Transaction")]
#[get("/{recurring_transaction_id}")]
pub(crate) async fn get_one_recurring_transaction(
    user: Phantom<User>,
    recurring_transaction_id: Path<Snowflake>,
) -> Result<impl Responder, ApiError> {
    let transaction = RecurringTransaction::find_by_id(recurring_transaction_id.into_inner()).await?;
    transaction.has_permission_or_error(user.get_id(), Permissions::READ).await?;

    Ok(HttpResponse::Ok().json(transaction))
}

#[utoipa::path(post,
responses(
(status = 200, description = "Successfully created Recurring Transaction.", content_type = "application/json", body = RecurringTransaction),
Unauthorized,
InternalServerError,
),
security(
("bearer_token" = [])
),
path = "/api/v1/transaction/recurring",
request_body = RecurringTransactionDTO,
tag = "Recurring-Transaction")]
#[post("")]
pub(crate) async fn create_recurring_transaction(
    user: Phantom<User>,
    mut recurring_transaction_dto: RecurringTransactionDTO,
) -> Result<impl Responder, ApiError> {
    recurring_transaction_dto
        .template_id
        .get_inner()
        .await?
        .has_permission_or_error(user.get_id(), Permissions::READ)
        .await?;

    let transaction = RecurringTransaction::new(recurring_transaction_dto).await?;

    Ok(HttpResponse::Created().json(transaction))
}

#[utoipa::path(delete,
    responses(
        (status = 204, description = "Successfully deleted Recurring Transaction."),
        Unauthorized,
        InternalServerError,
    ),
    security(
        ("bearer_token" = [])
    ),
    params(("recurring_transaction_id" = Snowflake,)),
    path = "/api/v1/transaction/recurring/{recurring_transaction_id}",
    tag = "Recurring-Transaction")]
#[delete("/{recurring_transaction_id}")]
pub(crate) async fn delete_recurring_transaction(
    user: Phantom<User>,
    recurring_transaction_id: Path<Snowflake>,
) -> Result<impl Responder, ApiError> {
    let transaction = RecurringTransaction::find_by_id(recurring_transaction_id.into_inner()).await?;
    transaction.has_permission_or_error(user.get_id(), Permissions::DELETE).await?;

    transaction.delete().await?;

    Ok(HttpResponse::NoContent())
}

#[utoipa::path(patch,
responses(
(status = 200, description = "Successfully updated Recurring Transaction.", content_type = "application/json", body = RecurringTransaction),
Unauthorized,
InternalServerError,
),
security(
("bearer_token" = [])
),
params(("recurring_transaction_id" = Snowflake,)),
path = "/api/v1/transaction/recurring/{recurring_transaction_id}",
request_body = RecurringTransactionDTO,
tag = "Recurring-Transaction")]
#[patch("/{recurring_transaction_id}")]
pub(crate) async fn update_recurring_transaction(
    user: Phantom<User>,
    recurring_transaction_id: Path<Snowflake>,
    recurring_transaction_dto: RecurringTransactionDTO,
) -> Result<impl Responder, ApiError> {
    let transaction = RecurringTransaction::find_by_id(recurring_transaction_id.into_inner()).await?;
    transaction.has_permission_or_error(user.get_id(), Permissions::READ_WRITE).await?;

    let transaction = transaction.update(recurring_transaction_dto).await?;

    Ok(HttpResponse::Ok().json(transaction))
}
