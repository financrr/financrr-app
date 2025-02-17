use crate::bank_account_linking::client::{GoCardlessClient, API_V2};
use crate::error::app_error::{AppError, AppResult};
use const_format::concatcp;
use serde::Deserialize;
use tracing::error;

#[derive(Debug, Clone, Deserialize)]
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
    pub async fn get_supported_institutions(&self, exclude_sandbox: bool) -> AppResult<Vec<Institution>> {
        const TEST_INSTITUTION_COUNTRY_CODE: &str = "xx";
        const URL_SUFFIX: &str = concatcp!(API_V2, "/institutions/");
        let url = Self::build_request_url(&self.config, URL_SUFFIX);

        let response = self.client.get(url).bearer_auth(self.get_token()).send().await?;
        match response.status().is_success() {
            false => {
                let status_code = response.status();
                let payload = response.text().await?;
                error!("Response failed. \nStatus code: {} \nPayload: {}", status_code, payload);

                Err(AppError::GeneralInternalServerError("".to_string()))
            }
            true => {
                let json: Vec<Institution> = response.json().await?;
                let json = json
                    .into_iter()
                    .map(|mut ins| {
                        // Convert to lowercase for later aggregation
                        ins.countries = ins.countries.into_iter().map(|c| c.to_lowercase()).collect();
                        ins
                    })
                    // Filter out test institutions if exclude_sandbox is true
                    .filter(|ins| !(exclude_sandbox && ins.countries.eq(&[TEST_INSTITUTION_COUNTRY_CODE])))
                    // Remove "xx" from countries if exclude_sandbox is true
                    .map(|mut ins| {
                        if exclude_sandbox {
                            ins.countries.retain(|c| c != TEST_INSTITUTION_COUNTRY_CODE);
                        }
                        ins
                    })
                    .collect();

                Ok(json)
            }
        }
    }
}
