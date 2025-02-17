use crate::bank_account_linking::client::{GoCardlessClient, API_V2};
use crate::error::app_error::{AppError, AppResult};
use const_format::concatcp;
use serde::{Deserialize, Serialize};
use tracing::error;

#[derive(Debug, Serialize)]
pub struct CreateRequisition {
    redirect: String,
    institution_id: String,
    agreement: String,
}

#[derive(Debug, Deserialize)]
pub struct Requisition {
    pub id: String,
    pub redirect: String,
    pub institution_id: String,
    pub agreement: String,
    #[serde(default)]
    pub accounts: Vec<String>,
    pub link: String,
}

impl GoCardlessClient {
    pub async fn create_requisition(
        &self,
        redirect: &str,
        institution_external_id: &str,
        agreement_external_id: &str,
    ) -> AppResult<Requisition> {
        let data = CreateRequisition {
            redirect: redirect.to_string(),
            institution_id: institution_external_id.to_string(),
            agreement: agreement_external_id.to_string(),
        };

        const URL_SUFFIX: &str = concatcp!(API_V2, "/requisitions");
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

    pub async fn get_requisition(&self, id: &str) -> AppResult<Requisition> {
        const URL_SUFFIX: &str = concatcp!(API_V2, "/requisitions");
        let url = format!("{URL_SUFFIX}/{id}");
        let url = Self::build_request_url(&self.config, url.as_str());

        let response = self.client.get(url).bearer_auth(self.get_token()).send().await?;

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
