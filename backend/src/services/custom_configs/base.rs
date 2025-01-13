use crate::services::custom_configs::bank_data_linking::BankDataLinkingConfig;
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

#[derive(Debug, Deserialize)]
pub struct CustomConfigInner {
    pub bank_data_linking: Option<BankDataLinkingConfig>,
    #[serde(skip)]
    is_bank_data_linking_configured: OnceLock<bool>,
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
        *self.is_bank_data_linking_configured.get_or_init(move || {
            let linking_config = match &self.bank_data_linking {
                Some(config) => config,
                None => return false,
            };

            if !linking_config.enabled || linking_config.implementations.is_empty() {
                return false;
            }

            for impls in &linking_config.implementations {
                if impls.is_enabled() {
                    return true;
                }
            }

            false
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::custom_configs::bank_data_linking::{GoCardlessConfig, LinkingImplementation};
    use loco_rs::environment::Environment;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_load_from_string() {
        let yaml = "";
        let path = "test_path.yaml".to_string();
        let result = CustomConfigInner::load_from_string(yaml.to_string(), path);
        assert!(result.is_ok());
    }

    #[test]
    fn test_load_from_path() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.yaml");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "").unwrap();

        let result = CustomConfigInner::load_from_path(file_path.to_str().unwrap().to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_load_from_env() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("development.yaml");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "").unwrap();

        env::set_var(CONFIG_FOLDER_ENV, dir.path().to_str().unwrap());
        let env = Environment::Development;
        let result = CustomConfigInner::load_from_env(&env);
        assert!(result.is_ok());
    }

    #[test]
    fn test_deserialize_custom_config_inner() {
        let yaml = r#"
    bank_data_linking:
      enabled: true
      implementations:
        - !GoCardless
          enabled: true
          secret_id: test_id
          secret_key: test_ke
    "#;

        let result: Result<CustomConfigInner, _> = serde_yaml::from_str(yaml);
        assert!(result.is_ok());

        let config = result.unwrap();
        assert!(config.bank_data_linking.is_some());
        let bank_account_linking = config.bank_data_linking.unwrap();
        assert!(bank_account_linking.enabled);
        assert_eq!(bank_account_linking.implementations.len(), 1);
    }

    #[test]
    fn test_is_bank_account_linking_configured() {
        // Case 1: Bank account linking is configured and enabled
        let config_with_linking_enabled = CustomConfigInner {
            bank_data_linking: Some(BankDataLinkingConfig {
                enabled: true,
                implementations: vec![LinkingImplementation::GoCardless(GoCardlessConfig {
                    enabled: true,
                    secret_id: "test_id".to_string(),
                    secret_key: "test_key".to_string(),
                })],
            }),
            is_bank_data_linking_configured: Default::default(),
        };
        assert!(config_with_linking_enabled.is_bank_data_linking_configured());

        // Case 2: Bank account linking is configured but not enabled
        let config_with_linking_disabled = CustomConfigInner {
            bank_data_linking: Some(BankDataLinkingConfig {
                enabled: false,
                implementations: vec![LinkingImplementation::GoCardless(GoCardlessConfig {
                    enabled: true,
                    secret_id: "test_id".to_string(),
                    secret_key: "test_key".to_string(),
                })],
            }),
            is_bank_data_linking_configured: Default::default(),
        };
        assert!(!config_with_linking_disabled.is_bank_data_linking_configured());

        // Case 3: Bank account linking is configured and enabled but no implementations
        let config_with_no_implementations = CustomConfigInner {
            bank_data_linking: Some(BankDataLinkingConfig {
                enabled: true,
                implementations: vec![],
            }),
            is_bank_data_linking_configured: Default::default(),
        };
        assert!(!config_with_no_implementations.is_bank_data_linking_configured());

        // Case 4: Bank account linking is not configured
        let config_without_linking = CustomConfigInner {
            bank_data_linking: None,
            is_bank_data_linking_configured: Default::default(),
        };
        assert!(!config_without_linking.is_bank_data_linking_configured());
    }
}
