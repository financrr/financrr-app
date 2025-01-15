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

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_debug_snapshot;
    use loco_rs::environment::Environment;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    macro_rules! configure_insta {
        ($($expr:expr),*) => {
            let mut settings = insta::Settings::clone_current();
            settings.set_prepend_module_to_snapshot(false);
            settings.set_snapshot_suffix("custom_config");
            let _guard = settings.bind_to_scope();
        };
    }

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
        writeln!(file).unwrap();

        let result = CustomConfigInner::load_from_path(file_path.to_str().unwrap().to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_load_from_env() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("development.yaml");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file).unwrap();

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
      secret_id: test_id
      secret_key: test_ke
    "#;

        let result: Result<CustomConfigInner, _> = serde_yaml::from_str(yaml);
        assert!(result.is_ok());

        let config = result.unwrap();
        assert!(config.bank_data_linking.is_some());
        let bank_account_linking = config.bank_data_linking.unwrap();
        assert!(bank_account_linking.enabled);
    }

    #[test]
    fn test_is_bank_account_linking_configured() {
        let config = CustomConfigInner {
            bank_data_linking: Some(BankDataLinkingConfig {
                enabled: true,
                secret_id: "test".to_string(),
                secret_key: "test".to_string(),
                api_url: "https://test.test".to_string(),
            }),
        };

        assert!(config.is_bank_data_linking_configured());

        let config = CustomConfigInner {
            bank_data_linking: None,
        };

        assert!(!config.is_bank_data_linking_configured());

        let config = CustomConfigInner {
            bank_data_linking: Some(BankDataLinkingConfig {
                enabled: false,
                secret_id: "test".to_string(),
                secret_key: "test".to_string(),
                api_url: "https://test.test".to_string(),
            }),
        };

        assert!(!config.is_bank_data_linking_configured());
    }

    #[test]
    fn test_default_values() {
        configure_insta!();

        let yaml = r#"
    bank_data_linking:
      enabled: true
      secret_id: test_id
      secret_key: test_key
      api_url: https://test.test
        "#;

        assert_yaml_snapshot(yaml);

        let yaml = r#"
    bank_data_linking:
      secret_id: test_id
      secret_key: test_key
      api_url: https://test.test
        "#;

        assert_yaml_snapshot(yaml);

        let yaml = r#"
    bank_data_linking:
      secret_id: test_id
      secret_key: test_key
        "#;

        assert_yaml_snapshot(yaml);
    }

    fn assert_yaml_snapshot(value: &str) {
        let result: Result<CustomConfigInner, _> = serde_yaml::from_str(value);
        assert!(result.is_ok());

        let config = result.unwrap();
        assert_debug_snapshot!(config);
    }
}
