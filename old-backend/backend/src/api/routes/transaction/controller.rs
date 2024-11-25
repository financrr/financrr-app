use actix_web::http::Uri;
use actix_web::web::{Json, Path};
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use actix_web_validation::Validated;
use utility::snowflake::entity::Snowflake;

use crate::api::documentation::response::{InternalServerError, Unauthorized, ValidationError};
use crate::api::error::api::ApiError;
use crate::api::pagination::{PageSizeParam, Pagination};
use crate::api::routes::transaction::recurring::controller::recurring_transaction_controller;
use crate::api::routes::transaction::template::controller::transaction_template_controller;
use crate::wrapper::entity::transaction::dto::{TransactionDTO, TransactionFromTemplate};
use crate::wrapper::entity::transaction::Transaction;
use crate::wrapper::entity::user::User;
use crate::wrapper::permission::{HasPermissionOrError, Permissions};
use crate::wrapper::types::phantom::{Identifiable, Phantom};

pub(crate) fn transaction_controller(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/transaction")
            .configure(transaction_template_controller)
            .configure(recurring_transaction_controller)
            .service(get_all_transactions)
            .service(create_transaction)
            .service(create_from_transaction_template)
            .service(delete_transaction)
            .service(update_transaction)
            .service(get_one_transaction),
    );
}

#[utoipa::path(get,
    responses(
        (status = 200, description = "Successfully retrieved all Transactions.", content_type = "application/json", body = PaginatedTransaction),
        ValidationError,
        Unauthorized,
        InternalServerError,
    ),
    params(PageSizeParam),
    security(
        ("bearer_token" = [])
    ),
    path = "/api/v1/transaction",
    tag = "Transaction")]
#[get("")]
pub(crate) async fn get_all_transactions(
    user: Phantom<User>,
    page_size: PageSizeParam,
    uri: Uri,
) -> Result<impl Responder, ApiError> {
    let total = Transaction::count_all_by_user(user.get_id()).await?;
    let transactions = Transaction::find_all_by_user_paginated(user.get_id(), &page_size).await?;

    Ok(HttpResponse::Ok().json(Pagination::new(transactions, &page_size, total, uri)))
}

#[utoipa::path(get,
    responses(
        (status = 200, description = "Successfully retrieved Transaction.", content_type = "application/json", body = Transaction),
        Unauthorized,
        InternalServerError,
    ),
    security(
        ("bearer_token" = [])
    ),
    params(("transaction_id" = Snowflake,)),
    path = "/api/v1/transaction/{transaction_id}",
    tag = "Transaction")]
#[get("/{transaction_id}")]
pub(crate) async fn get_one_transaction(
    user: Phantom<User>,
    transaction_id: Path<Snowflake>,
) -> Result<impl Responder, ApiError> {
    let transaction_id = transaction_id.into_inner();
    let transaction = Transaction::find_by_id(transaction_id).await?;
    transaction.has_permission_or_error(user.get_id(), Permissions::READ).await?;

    Ok(HttpResponse::Ok().json(transaction))
}

#[utoipa::path(post,
responses(
(status = 201, description = "Successfully created Transaction.", content_type = "application/json", body = Transaction),
Unauthorized,
InternalServerError,
),
security(
("bearer_token" = [])
),
path = "/api/v1/transaction",
request_body = TransactionDTO,
tag = "Transaction")]
#[post("")]
pub(crate) async fn create_transaction(
    user: Phantom<User>,
    transaction: Validated<Json<TransactionDTO>>,
) -> Result<impl Responder, ApiError> {
    let transaction = transaction.into_inner().into_inner();

    if !transaction.check_permissions(user.get_id()).await? {
        return Err(ApiError::Unauthorized());
    }

    Ok(HttpResponse::Created().json(Transaction::new(transaction).await?))
}

#[utoipa::path(post,
responses(
(status = 201, description = "Successfully created Transaction from Template.", content_type = "application/json", body = Transaction),
Unauthorized,
InternalServerError,
),
security(
("bearer_token" = [])
),
path = "/api/v1/transaction/from-template",
request_body = TransactionFromTemplate,
tag = "Transaction")]
#[post("/from-template")]
pub(crate) async fn create_from_transaction_template(
    user: Phantom<User>,
    transaction_from_template: Validated<Json<TransactionFromTemplate>>,
) -> Result<impl Responder, ApiError> {
    let mut transaction_from_template = transaction_from_template.into_inner().into_inner();

    let template = transaction_from_template.template_id.get_inner().await?;
    template.has_permission_or_error(user.get_id(), Permissions::READ).await?;
    let dto = TransactionDTO::from_template(template, transaction_from_template.executed_at).await?;

    Ok(HttpResponse::Created().json(Transaction::new(dto).await?))
}

#[utoipa::path(delete,
    responses(
        (status = 204, description = "Successfully deleted Transaction."),
        Unauthorized,
        InternalServerError,
    ),
    security(
        ("bearer_token" = [])
    ),
    params(("transaction_id" = Snowflake,)),
    path = "/api/v1/transaction/{transaction_id}",
    tag = "Transaction")]
#[delete("/{transaction_id}")]
pub(crate) async fn delete_transaction(
    user: Phantom<User>,
    transaction_id: Path<Snowflake>,
) -> Result<impl Responder, ApiError> {
    let transaction_id = transaction_id.into_inner();
    let transaction = Transaction::find_by_id(transaction_id).await?;
    transaction.has_permission_or_error(user.get_id(), Permissions::READ_DELETE).await?;

    transaction.delete().await?;

    Ok(HttpResponse::NoContent())
}

#[utoipa::path(patch,
responses(
(status = 200, description = "Successfully updated Transaction.", content_type = "application/json", body = Transaction),
Unauthorized,
InternalServerError,
),
security(
("bearer_token" = [])
),
params(("transaction_id" = Snowflake,)),
path = "/api/v1/transaction/{transaction_id}",
request_body = TransactionDTO,
tag = "Transaction")]
#[patch("/{transaction_id}")]
pub(crate) async fn update_transaction(
    user: Phantom<User>,
    transaction_dto: Validated<Json<TransactionDTO>>,
    transaction_id: Path<Snowflake>,
) -> Result<impl Responder, ApiError> {
    let transaction_dto = transaction_dto.into_inner().into_inner();

    let transaction = Transaction::find_by_id(transaction_id.into_inner()).await?;
    transaction.has_permission_or_error(user.get_id(), Permissions::READ_WRITE).await?;

    let transaction = transaction.update(transaction_dto).await?;

    Ok(HttpResponse::Ok().json(transaction))
}
