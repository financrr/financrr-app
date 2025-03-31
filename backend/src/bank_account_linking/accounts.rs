use crate::bank_account_linking::client::{API_V2, GoCardlessClient};
use crate::error::app_error::AppResult;
use chrono::{DateTime, FixedOffset};
use const_format::concatcp;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BankAccountInformation {
    pub id: String,
    pub created: DateTime<FixedOffset>,
    pub last_accessed: DateTime<FixedOffset>,
    pub iban: String,
    pub status: String,
    pub institution_id: String,
    pub owner_name: String,
}

impl GoCardlessClient {
    pub async fn get_account_information(&self, id: &str) -> AppResult<BankAccountInformation> {
        const URL_SUFFIX: &str = concatcp!(API_V2, "/accounts");
        let url = format!("{}/{}", URL_SUFFIX, id);
        let url = Self::build_request_url(&self.config, url.as_str());

        let response = self.client.get(url).send().await?;

        self.parse_response(response).await
    }
}
