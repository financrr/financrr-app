use actix_web::dev::Payload;
use actix_web::error::QueryPayloadError;
use actix_web::http::Uri;
use actix_web::web::Query;
use actix_web::{FromRequest, HttpRequest};
use futures_util::future::LocalBoxFuture;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::openapi::path::{Parameter, ParameterBuilder, ParameterIn};
use utoipa::openapi::{KnownFormat, ObjectBuilder, Required, SchemaFormat, Type};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

use crate::api::error::api::ApiError;
use crate::entity::account_entity::Account;
use crate::wrapper::entity::budget::Budget;
use crate::wrapper::entity::currency::Currency;
use crate::wrapper::entity::session::Session;
use crate::wrapper::entity::transaction::recurring::RecurringTransaction;
use crate::wrapper::entity::transaction::template::TransactionTemplate;
use crate::wrapper::entity::transaction::Transaction;

pub(crate) const DEFAULT_PAGE: u64 = 1;
pub(crate) const DEFAULT_LIMIT: u64 = 50;
pub(crate) const MAX_LIMIT: u64 = 500;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, ToSchema)]
#[aliases(
    PaginatedAccount = Pagination < Account >,
    PaginatedBudget = Pagination < Budget >,
    PaginatedCurrency = Pagination < Currency >,
    PaginatedTransaction = Pagination < Transaction >,
    PaginatedTransactionTemplate = Pagination < TransactionTemplate >,
    PaginatedRecurringTransaction = Pagination < RecurringTransaction >,
    PaginatedSession = Pagination < Session >,
)]
pub(crate) struct Pagination<T: Serialize + ToSchema<'static>> {
    #[serde(rename = "_metadata")]
    pub(crate) metadata: Metadata,
    pub(crate) data: Vec<T>,
}

impl<T: Serialize + ToSchema<'static>> Pagination<T> {
    pub(crate) fn new(data: Vec<T>, page_size_param: &PageSizeParam, total: u64, uri: Uri) -> Self {
        Self {
            metadata: Metadata {
                page: page_size_param.page,
                limit: page_size_param.limit,
                total,
                links: Links::new(uri, page_size_param, total),
            },
            data,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, ToSchema)]
pub(crate) struct Metadata {
    pub(crate) page: u64,
    pub(crate) limit: u64,
    pub(crate) total: u64,
    pub(crate) links: Links,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, ToSchema)]
pub(crate) struct Links {
    pub(crate) prev: Option<String>,
    pub(crate) next: Option<String>,
}

impl Links {
    pub(crate) fn new(uri: Uri, page_size_param: &PageSizeParam, total: u64) -> Self {
        let page = page_size_param.page;
        let limit = page_size_param.limit;

        // only show prev link if he isn't on the first page
        let prev = if page > 1 {
            Some(format!("{}?page={}&limit={}", uri.path(), page - 1, limit))
        } else {
            None
        };

        // only show next link if there are more items
        let next = if total > page * limit {
            Some(format!("{}?page={}&limit={}", uri.path(), page + 1, limit))
        } else {
            None
        };

        Self {
            prev,
            next,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Validate, Deserialize)]
pub(crate) struct PageSizeParam {
    #[validate(range(min = 1))]
    pub(crate) page: u64,
    #[validate(range(min = 1, max = "MAX_LIMIT"))]
    pub(crate) limit: u64,
}

impl PageSizeParam {
    pub(crate) fn new(page: u64, limit: u64) -> Self {
        Self {
            page,
            limit,
        }
    }
}

impl Default for PageSizeParam {
    fn default() -> Self {
        Self {
            page: DEFAULT_PAGE,
            limit: DEFAULT_LIMIT,
        }
    }
}

impl FromRequest for PageSizeParam {
    type Error = ApiError;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let page_size_params: Result<Query<Self>, QueryPayloadError> = Query::from_query(req.query_string());

        let page_size_params = match page_size_params {
            Ok(page_size_params) => page_size_params.into_inner(),
            Err(_) => Self::default(),
        };

        Box::pin(async move {
            page_size_params.validate().map_err(ApiError::from)?;

            Ok(page_size_params)
        })
    }
}

impl IntoParams for PageSizeParam {
    fn into_params(_parameter_in_provider: impl Fn() -> Option<ParameterIn>) -> Vec<Parameter> {
        vec![
            ParameterBuilder::new()
                .name("page")
                .parameter_in(ParameterIn::Query)
                .required(Required::False)
                .description(Some("The page number."))
                .schema(Some(
                    ObjectBuilder::new()
                        .default(Some(Value::from(DEFAULT_PAGE)))
                        .schema_type(Type::Integer)
                        .format(Some(SchemaFormat::KnownFormat(KnownFormat::Int64)))
                        .build(),
                ))
                .build(),
            ParameterBuilder::new()
                .name("limit")
                .parameter_in(ParameterIn::Query)
                .required(Required::False)
                .description(Some("The number of items per page"))
                .schema(Some(
                    ObjectBuilder::new()
                        .default(Some(Value::from(DEFAULT_LIMIT)))
                        .schema_type(Type::Integer)
                        .format(Some(SchemaFormat::KnownFormat(KnownFormat::Int64)))
                        .build(),
                ))
                .build(),
        ]
    }
}
