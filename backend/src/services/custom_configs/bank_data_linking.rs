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

impl LinkingImplementation {
    pub fn is_enabled(&self) -> bool {
        match self {
            LinkingImplementation::GoCardless(conf) => conf.enabled,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoCardlessConfig {
    pub enabled: bool,
    pub secret_id: String,
    pub secret_key: String,
}

fn default_enabled() -> bool {
    true
}
