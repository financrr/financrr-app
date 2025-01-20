use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct BankDataLinkingConfig {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    pub secret_id: String,
    pub secret_key: String,
    #[serde(default = "default_go_cardless_api_url")]
    pub api_url: String,
}

fn default_enabled() -> bool {
    true
}

fn default_go_cardless_api_url() -> String {
    "https://bankaccountdata.gocardless.com/".to_string()
}
