use actix_web::http::Uri;
use actix_web::web::Json;
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use actix_web_validation::Validated;
use web::{Path, ServiceConfig};

use utility::snowflake::entity::Snowflake;

use crate::api::documentation::response::{InternalServerError, Unauthorized, ValidationError};
use crate::api::error::api::ApiError;
use crate::api::pagination::{PageSizeParam, Pagination};
use crate::wrapper::entity::transaction::template::dto::TransactionTemplateDTO;
use crate::wrapper::entity::transaction::template::TransactionTemplate;
use crate::wrapper::entity::user::User;
use crate::wrapper::permission::{HasPermissionOrError, Permissions};
use crate::wrapper::types::phantom::{Identifiable, Phantom};

pub(crate) fn transaction_template_controller(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/template")
            .service(get_all_transaction_templates)
            .service(get_one_transaction_template)
            .service(create_transaction_template)
            .service(delete_transaction_template)
            .service(update_transaction_template),
    );
}

#[utoipa::path(get,
    responses(
        (status = 200, description = "Successfully retrieved all Transactions.", content_type = "application/json", body = PaginatedTransactionTemplate),
        ValidationError,
        Unauthorized,
        InternalServerError,
    ),
    params(PageSizeParam),
    security(
        ("bearer_token" = [])
    ),
    path = "/api/v1/transaction/template",
    tag = "Transaction-Template")]
#[get("")]
pub(crate) async fn get_all_transaction_templates(
    user: Phantom<User>,
    page_size: PageSizeParam,
    uri: Uri,
) -> Result<impl Responder, ApiError> {
    let total = TransactionTemplate::count_all_by_user_id(user.get_id()).await?;
    let transactions = TransactionTemplate::find_all_by_user_id_paginated(user.get_id(), &page_size).await?;

    Ok(HttpResponse::Ok().json(Pagination::new(transactions, &page_size, total, uri)))
}

#[utoipa::path(get,
    responses(
        (status = 200, description = "Successfully retrieved Transaction.", content_type = "application/json", body = TransactionTemplate),
        Unauthorized,
        InternalServerError,
    ),
    security(
        ("bearer_token" = [])
    ),
    params(("template_id" = Snowflake,)),
    path = "/api/v1/transaction/template/{template_id}",
    tag = "Transaction-Template")]
#[get("/{template_id}")]
pub(crate) async fn get_one_transaction_template(
    user: Phantom<User>,
    template_id: Path<Snowflake>,
) -> Result<impl Responder, ApiError> {
    let transaction = TransactionTemplate::find_by_id(template_id.into_inner()).await?;
    transaction.has_permission_or_error(user.get_id(), Permissions::READ).await?;

    Ok(HttpResponse::Ok().json(transaction))
}

#[utoipa::path(post,
responses(
(status = 201, description = "Successfully created TransactionTemplate.", content_type = "application/json", body = TransactionTemplate),
ValidationError,
Unauthorized,
InternalServerError,
),
security(
("bearer_token" = [])
),
path = "/api/v1/transaction/template",
request_body = TransactionTemplateDTO,
tag = "Transaction-Template")]
#[post("")]
pub(crate) async fn create_transaction_template(
    user: Phantom<User>,
    template: Validated<Json<TransactionTemplateDTO>>,
) -> Result<impl Responder, ApiError> {
    let template = template.into_inner().into_inner();

    if !template.check_permissions(user.get_id()).await? {
        return Err(ApiError::Unauthorized());
    }

    Ok(HttpResponse::Created().json(TransactionTemplate::new(template, user.get_id()).await?))
}

#[utoipa::path(delete,
    responses(
        (status = 204, description = "Successfully deleted the TransactionTemplate."),
        Unauthorized,
        InternalServerError,
    ),
    security(
        ("bearer_token" = [])
    ),
    params(("template_id" = Snowflake,)),
    path = "/api/v1/transaction/template/{template_id}",
    tag = "Transaction-Template")]
#[delete("/{template_id}")]
pub(crate) async fn delete_transaction_template(
    user: Phantom<User>,
    template_id: Path<Snowflake>,
) -> Result<impl Responder, ApiError> {
    let template = TransactionTemplate::find_by_id(template_id.into_inner()).await?;
    template.has_permission_or_error(user.get_id(), Permissions::READ_DELETE).await?;

    template.delete().await?;

    Ok(HttpResponse::NoContent())
}

#[utoipa::path(patch,
responses(
(status = 200, description = "Successfully updated the TransactionTemplate.", content_type = "application/json", body = TransactionTemplate),
Unauthorized,
InternalServerError,
),
security(
("bearer_token" = [])
),
params(("template_id" = Snowflake,)),
path = "/api/v1/transaction/template/{template_id}",
request_body = TransactionTemplateDTO,
tag = "Transaction-Template")]
#[patch("/{template_id}")]
pub(crate) async fn update_transaction_template(
    user: Phantom<User>,
    update: Validated<Json<TransactionTemplateDTO>>,
    template_id: Path<Snowflake>,
) -> Result<impl Responder, ApiError> {
    let update = update.into_inner().into_inner();

    let template = TransactionTemplate::find_by_id(template_id.into_inner()).await?;
    template.has_permission_or_error(user.get_id(), Permissions::READ_WRITE).await?;

    let template = template.update(update).await?;

    Ok(HttpResponse::Ok().json(template))
}
