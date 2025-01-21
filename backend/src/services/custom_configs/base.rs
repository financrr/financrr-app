use crate::services::custom_configs::bank_data_linking::BankDataLinkingConfig;
use crate::services::custom_configs::opensearch::OpensearchConfig;
use crate::services::Service;
use loco_rs::app::AppContext;
use loco_rs::environment::Environment;
use loco_rs::Error;
use serde::Deserialize;
use std::env;
use std::fs::read_to_string;
use std::sync::{Arc, OnceLock};
use tera::{Context, Tera};
use tracing::{debug, error};

pub const CONFIG_FOLDER_ENV: &str = "LOCO_CONFIG_FOLDER";
pub const DEFAULT_CONFIG_FOLDER: &str = "config";

pub type CustomConfig = Arc<CustomConfigInner>;

#[derive(Debug, Clone, Deserialize)]
pub struct CustomConfigInner {
    pub bank_data_linking: Option<BankDataLinkingConfig>,
    pub opensearch: OpensearchConfig,
}

impl Service for CustomConfigInner {
    async fn new(ctx: &AppContext) -> loco_rs::Result<Self> {
        Self::load_from_env(&ctx.environment)
    }

    fn get_static_once() -> &'static OnceLock<Arc<Self>> {
        static INSTANCE: OnceLock<CustomConfig> = OnceLock::new();

        &INSTANCE
    }
}

impl CustomConfigInner {
    pub fn get_config_folder() -> String {
        env::var(CONFIG_FOLDER_ENV).unwrap_or(DEFAULT_CONFIG_FOLDER.to_string())
    }

    fn load_from_env(env: &Environment) -> loco_rs::Result<Self> {
        let file_name = env.to_string();
        let path = {
            let mut path = Self::get_config_folder();
            if !path.ends_with("/") {
                path.push('/');
            }
            path.push_str(file_name.as_str());
            path.push_str(".yaml");

            path
        };

        debug!("Loading extended config from path: {}", path);

        Self::load_from_path(path)
    }

    fn load_from_path(path: String) -> loco_rs::Result<Self> {
        Self::load_from_string(read_to_string(path.as_str())?, path)
    }

    fn load_from_string(yaml: String, path: String) -> loco_rs::Result<Self> {
        let rendered = Tera::one_off(yaml.as_str(), &Context::new(), false)?;

        serde_yml::from_str(&rendered).map_err(|err| {
            error!("Yaml Error: {} | File: {}", err.to_string(), path);

            Error::Any(err.into())
        })
    }

    pub fn is_bank_data_linking_configured(&self) -> bool {
        if let Some(conf) = &self.bank_data_linking {
            return conf.enabled;
        }

        false
    }
}
