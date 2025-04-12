use crate::models::external_bank_institutions::Model;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexableExternalBankInstitution {
    pub id: i64,
    pub provider: String,
    pub name: String,
    pub bic: Option<String>,
    pub countries: Vec<String>,
    pub logo_link: Option<String>,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

impl From<Model> for IndexableExternalBankInstitution {
    fn from(value: Model) -> Self {
        Self {
            id: value.id,
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
