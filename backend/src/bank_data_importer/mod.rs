use crate::error::app_error::AppResult;
use async_trait::async_trait;
use loco_rs::prelude::AppContext;
use std::fmt::Debug;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BankDataImporterError {
    #[error("There is no available implementations.")]
    NoneAvailableImplementations,
}

// TODO: migrate to shared_store with loco 0.16
pub struct BankDataImporterRegistry {
    registry: [Box<dyn BankDataImporter>; 0],
}

impl BankDataImporterRegistry {
    pub async fn build(_ctx: &AppContext) -> AppResult<Self> {
        Ok(Self { registry: [] })
    }

    pub async fn get_one_available_implementation(&self) -> AppResult<&Box<dyn BankDataImporter>> {
        for importer in &self.registry {
            if importer.is_available().await? {
                return Ok(importer);
            }
        }

        Err(BankDataImporterError::NoneAvailableImplementations.into())
    }
}

/// The flows we support to link an account with that specific importer/provider.
#[derive(Debug)]
pub enum LinkingFlow {
    /// Normal Oauth2 flow.
    Oauth2,
}

#[async_trait]
pub trait BankDataImporter {
    /// Returns true if the implementation can handle at least one more client.
    async fn is_available(&self) -> AppResult<bool>;

    fn get_linking_flows(&self) -> LinkingFlow;

    async fn get_oauth2_link(&self) -> AppResult<String>;
}
