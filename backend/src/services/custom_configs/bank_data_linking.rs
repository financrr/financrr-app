use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct BankDataLinkingConfig {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    pub implementations: Vec<LinkingImplementation>,
}

#[derive(Debug, Deserialize)]
pub enum LinkingImplementation {
    GoCardless(GoCardlessConfig),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoCardlessConfig {
    pub enabled: bool,
    pub secret_id: String,
    pub secret_key: String,
    #[serde(default = "default_go_cardless_api_url")]
    pub api_url: String,
}

impl LinkingImplementation {
    pub fn is_enabled(&self) -> bool {
        match self {
            LinkingImplementation::GoCardless(conf) => conf.enabled,
        }
    }
}

fn default_enabled() -> bool {
    true
}

fn default_go_cardless_api_url() -> String {
    "https://bankaccountdata.gocardless.com/".to_string()
}
