use crate::bank_account_linking::client::{GoCardlessClient, API_V2};
use crate::error::app_error::{AppError, AppResult};
use const_format::concatcp;
use serde::Deserialize;
use tracing::error;

#[derive(Debug, Deserialize)]
pub struct Institution {
    pub id: String,
    pub name: String,
    pub bic: Option<String>,
    pub transaction_total_days: String,
    pub max_access_valid_for_days: String,
    pub countries: Vec<String>,
    pub logo: Option<String>,
}

impl GoCardlessClient {
    pub async fn get_supported_institutions(&self) -> AppResult<Vec<Institution>> {
        const URL_SUFFIX: &str = concatcp!(API_V2, "/institutions/");
        let url = Self::build_request_url(&self.config, URL_SUFFIX);

        let response = self.client.get(url).bearer_auth(self.get_token()).send().await?;
        match response.status().as_u16() > 299 {
            true => {
                let status_code = response.status();
                let payload = response.text().await?;
                error!("Response failed. \nStatus code: {} \nPayload: {}", status_code, payload);

                Err(AppError::GeneralInternalServerError("".to_string()))
            }
            false => Ok(response.json().await?),
        }
    }
}
