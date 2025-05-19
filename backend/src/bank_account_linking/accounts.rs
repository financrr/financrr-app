use crate::bank_account_linking::client::{API_V2, GoCardlessClient};
use crate::error::app_error::AppResult;
use chrono::{DateTime, FixedOffset, NaiveDate};
use const_format::concatcp;
use serde::Deserialize;
use tokio::try_join;

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

#[derive(Debug, Deserialize)]
pub struct BankAccountDetails {
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    pub iban: String,
    pub currency: String,
    #[serde(rename = "ownerName")]
    pub owner_name: String,
    pub name: String,
    pub product: String,
    #[serde(rename = "cashAccountType")]
    pub cash_account_type: String,
}

#[derive(Debug, Deserialize)]
pub struct AccountBalances {
    pub balances: Vec<Balance>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    pub balance_amount: BalanceAmount,
    pub balance_type: BalanceType,
    pub reference_date: NaiveDate,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BalanceAmount {
    pub amount: String,
    pub currency: String,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum BalanceType {
    Expected,
    #[serde(rename = "interimAvailable")]
    InterimAvailable,
}

impl GoCardlessClient {
    pub async fn get_account_overview(
        &self,
        id: &str,
    ) -> AppResult<(BankAccountInformation, BankAccountDetails, AccountBalances)> {
        Ok(try_join!(
            self.get_account_information(id),
            self.get_account_details(id),
            self.get_account_balances(id)
        )?)
    }

    pub async fn get_account_information(&self, id: &str) -> AppResult<BankAccountInformation> {
        const URL_SUFFIX: &str = concatcp!(API_V2, "/accounts");
        let url = format!("{}/{}", URL_SUFFIX, id);
        let url = Self::build_request_url(&self.config, url.as_str());

        let response = self.client.get(url).send().await?;

        self.parse_response(response).await
    }

    pub async fn get_account_details(&self, id: &str) -> AppResult<BankAccountDetails> {
        const URL_SUFFIX: &str = concatcp!(API_V2, "/accounts");
        let url = format!("{}/{}/details", URL_SUFFIX, id);
        let url = Self::build_request_url(&self.config, url.as_str());

        let response = self.client.get(url).send().await?;

        self.parse_response(response).await
    }

    pub async fn get_account_balances(&self, id: &str) -> AppResult<AccountBalances> {
        const URL_SUFFIX: &str = concatcp!(API_V2, "/accounts");
        let url = format!("{}/{}/balances", URL_SUFFIX, id);
        let url = Self::build_request_url(&self.config, url.as_str());

        let response = self.client.get(url).send().await?;

        self.parse_response(response).await
    }
}
