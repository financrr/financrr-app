use actix_web::dev::Payload;
use actix_web::web::Json;
use actix_web::FromRequest;
use futures_util::future::LocalBoxFuture;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::api::error::api::ApiError;
use crate::wrapper::entity::transaction::recurring::recurring_rule::dto::RecurringRuleDTO;
use crate::wrapper::entity::transaction::recurring::validation::assert_template_exists;
use crate::wrapper::entity::transaction::template::TransactionTemplate;
use crate::wrapper::types::phantom::Phantom;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema, Validate)]
pub(crate) struct RecurringTransactionDTO {
    #[validate(custom(function = "assert_template_exists"))]
    pub(crate) template_id: Phantom<TransactionTemplate>,
    #[validate(nested)]
    pub(crate) recurring_rule: RecurringRuleDTO,
}

impl FromRequest for RecurringTransactionDTO {
    type Error = ApiError;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, payload: &mut Payload) -> Self::Future {
        let json_fut = Json::<Self>::from_request(req, payload);
        Box::pin(async move {
            let dto = json_fut.await?;
            let dto = dto.into_inner();

            dto.validate().map_err(ApiError::from)?;

            Ok(dto)
        })
    }
}
