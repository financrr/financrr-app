use crate::models::external_bank_institutions::Model;
use crate::types::snowflake::Snowflake;
use crate::views::pagination::{Pager, PagerMeta};
use chrono::{DateTime, FixedOffset};
use loco_rs::model::query::{PageResponse, PaginationQuery};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct ExternalBankInstitutionResponse {
    pub id: Snowflake,
    pub provider: String,
    pub name: String,
    pub bic: Option<String>,
    pub countries: Vec<String>,
    pub logo_link: Option<String>,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

impl From<Model> for ExternalBankInstitutionResponse {
    fn from(value: Model) -> Self {
        Self {
            id: Snowflake::new(value.id),
            provider: value.provider,
            name: value.name,
            bic: value.bic,
            countries: value.countries,
            logo_link: value.logo_link,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl ExternalBankInstitutionResponse {
    pub fn response(data: PageResponse<Model>, query: &PaginationQuery) -> Pager<ExternalBankInstitutionResponse> {
        Pager {
            results: data
                .page
                .into_iter()
                .map(ExternalBankInstitutionResponse::from)
                .collect::<Vec<Self>>(),
            info: PagerMeta {
                page: query.page,
                page_size: query.page_size,
                total_pages: data.total_pages,
                // TODO fix!
                total: 0,
            },
        }
    }
}
