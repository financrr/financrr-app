use crate::bank_account_linking::client::{GoCardlessClient, API_V2};
use crate::bank_account_linking::constants::DEFAULT_TRANSACTION_TOTAL_DAYS;
use crate::error::app_error::{AppError, AppResult};
use chrono::{DateTime, FixedOffset};
use const_format::concatcp;
use serde::{Deserialize, Serialize};
use tracing::error;

#[derive(Debug, Serialize)]
pub struct CreateEndUserAgreement {
    pub institution_id: String,
    pub max_historical_days: u16,
    pub access_valid_for_days: u16,
}

#[derive(Debug, Deserialize)]
pub struct EndUserAgreement {
    pub id: String,
    pub created: DateTime<FixedOffset>,
    pub institution_id: String,
    pub max_historical_days: u16,
    pub access_valid_for_days: u16,
    pub accepted: Option<DateTime<FixedOffset>>,
}

impl GoCardlessClient {
    pub async fn create_end_user_agreement(
        &self,
        institution_id: &str,
        access_valid_for_days: u16,
    ) -> AppResult<EndUserAgreement> {
        let data = CreateEndUserAgreement {
            institution_id: institution_id.to_string(),
            max_historical_days: DEFAULT_TRANSACTION_TOTAL_DAYS,
            access_valid_for_days,
        };

        const URL_SUFFIX: &str = concatcp!(API_V2, "/agreements/enduser");
        let url = Self::build_request_url(&self.config, URL_SUFFIX);

        let response = self
            .client
            .post(url)
            .json(&data)
            .bearer_auth(self.get_token())
            .send()
            .await?;

        match response.status().is_success() {
            false => {
                let status_code = response.status();
                let payload = response.text().await?;
                error!("Response failed. \nStatus code: {} \nPayload: {}", status_code, payload);

                Err(AppError::GeneralInternalServerError("".to_string()))
            }
            true => Ok(response.json().await?),
        }
    }

    pub async fn get_end_user_agreement(&self, id: &str) -> AppResult<EndUserAgreement> {
        const URL_SUFFIX: &str = concatcp!(API_V2, "/agreements/enduser");
        let url = format!("{URL_SUFFIX}/{id}");
        let url = Self::build_request_url(&self.config, url.as_str());

        let response = self.client.get(url).send().await?;

        match response.status().is_success() {
            false => {
                let status_code = response.status();
                let payload = response.text().await?;
                error!("Response failed. \nStatus code: {} \nPayload: {}", status_code, payload);

                Err(AppError::GeneralInternalServerError("".to_string()))
            }
            true => Ok(response.json().await?),
        }
    }
}
