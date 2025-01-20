use crate::bank_account_linking::client::GoCardlessClient;
use crate::services::custom_configs::base::CustomConfigInner;
use crate::services::Service;
use loco_rs::app::AppContext;
use std::sync::{Arc, OnceLock};

pub type BankLinkingData = Arc<BankLinkingDataInner>;

#[derive(Debug, Default)]
pub struct BankLinkingDataInner {
    go_cardless_client: Option<Arc<GoCardlessClient>>,
}

impl Service for BankLinkingDataInner {
    async fn new(ctx: &AppContext) -> loco_rs::Result<Self> {
        let config = CustomConfigInner::get_arc(ctx).await?;
        if !config.is_bank_data_linking_configured() {
            return Ok(BankLinkingDataInner::default());
        }

        if let Some(conf) = config.bank_data_linking.clone() {
            let client = GoCardlessClient::init(conf).await?;

            return Ok(Self {
                go_cardless_client: Some(Arc::new(client)),
            });
        }

        Ok(BankLinkingDataInner::default())
    }

    fn get_static_once() -> &'static OnceLock<Arc<Self>> {
        static INSTANCE: OnceLock<BankLinkingData> = OnceLock::new();

        &INSTANCE
    }
}

impl BankLinkingDataInner {
    pub fn get_go_cardless_client(&self) -> Option<Arc<GoCardlessClient>> {
        self.go_cardless_client.clone()
    }

    pub fn is_go_cardless_client_configured(&self) -> bool {
        match self.get_go_cardless_client() {
            None => false,
            Some(client) => client.is_enabled(),
        }
    }
}
