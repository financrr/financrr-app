use crate::bank_account_linking::responses::{JwtResponse, RefreshResponse};
use crate::error::app_error::{AppError, AppResult};
use crate::factories::bank_account_factory::BankAccountFactory;
use crate::services::Service;
use crate::services::custom_configs::bank_data_linking::BankDataLinkingConfig;
use const_format::concatcp;
use loco_rs::prelude::AppContext;
use parking_lot::RwLock;
use reqwest::{Client, Response};
use serde::de::DeserializeOwned;
use serde_json::json;
use std::sync::Arc;
use std::time::Duration;
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};
use tracing::error;

pub const API_V2: &str = "/api/v2";

#[derive(Debug)]
pub struct GoCardlessClient {
    pub(super) config: BankDataLinkingConfig,
    pub(super) token: Arc<RwLock<JwtResponse>>,
    pub(super) client: Client,
    pub(super) bank_account_factory: Arc<BankAccountFactory>,
}

impl GoCardlessClient {
    pub async fn init(ctx: &AppContext, config: BankDataLinkingConfig) -> AppResult<Self> {
        let client = GoCardlessClient {
            config,
            token: Arc::new(RwLock::new(JwtResponse::default())),
            client: Client::new(),
            bank_account_factory: BankAccountFactory::get_arc(ctx).await?,
        };
        let token = client.create_new_token().await?;
        *client.token.write() = token;

        client.start_token_renewal().await.map_err(|err| {
            error!(
                "An error occurred while trying to schedule GoCardless token renewal. Error: {}",
                err.to_string()
            );

            AppError::GeneralInternalServerError(err.to_string())
        })?;

        Ok(client)
    }

    async fn start_token_renewal(&self) -> Result<(), JobSchedulerError> {
        let scheduler = JobScheduler::new().await?;

        let secs = self.token.read().access_expires - 5;
        let token = self.token.clone();
        let client = self.client.clone();
        let config = self.config.clone();
        let job = Job::new_repeated_async(Duration::from_secs(secs as u64), move |_uuid, _l| {
            let token = token.clone();
            let client = client.clone();
            let config = config.clone();
            Box::pin(async move {
                let refresh_token = token.read().refresh.clone();
                match Self::refresh_token(refresh_token.as_str(), &config, &client).await {
                    Ok(res) => {
                        let mut token = token.write();
                        token.access = res.access;
                        token.access_expires = res.access_expires;
                    }
                    Err(err) => error!("Error while trying to renew token. Error: {}", err),
                }
            })
        })?;

        scheduler.add(job).await?;

        scheduler.shutdown_on_ctrl_c();
        scheduler.start().await?;

        Ok(())
    }

    async fn create_new_token(&self) -> AppResult<JwtResponse> {
        const URL_SUFFIX: &str = concatcp!(API_V2, "/token/new/");
        let url = Self::build_request_url(&self.config, URL_SUFFIX);

        let payload = json!({
            "secret_id": self.config.secret_id,
            "secret_key": self.config.secret_key
        });

        let response = self.client.post(url).json(&payload).send().await?;

        Ok(response.json::<JwtResponse>().await?)
    }

    async fn refresh_token(
        refresh: &str,
        config: &BankDataLinkingConfig,
        client: &Client,
    ) -> AppResult<RefreshResponse> {
        const URL_SUFFIX: &str = concatcp!(API_V2, "/token/refresh/");
        let url = Self::build_request_url(config, URL_SUFFIX);

        let payload = json!({
            "refresh": refresh
        });
        let response = client.post(url).json(&payload).send().await?;

        Ok(response.json::<RefreshResponse>().await?)
    }

    pub(super) fn get_token(&self) -> String {
        self.token.read().access.clone()
    }

    pub(super) fn build_request_url(config: &BankDataLinkingConfig, endpoint: &str) -> String {
        let api_url = match config.api_url.strip_suffix("/") {
            None => config.api_url.to_owned(),
            Some(url) => url.to_owned(),
        };

        let endpoint = match endpoint.starts_with("/") {
            true => endpoint.to_owned(),
            false => format!("/{}", endpoint),
        };

        let endpoint = match endpoint.ends_with("/") {
            true => endpoint,
            false => format!("{}/", endpoint),
        };

        format!("{}{}", api_url, endpoint)
    }

    pub fn is_enabled(&self) -> bool {
        self.config.enabled
    }

    pub async fn parse_response<T: DeserializeOwned>(&self, response: Response) -> AppResult<T> {
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
