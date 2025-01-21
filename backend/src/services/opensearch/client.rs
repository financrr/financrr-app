use crate::error::app_error::AppResult;
use crate::services::custom_configs::base::CustomConfigInner;
use crate::services::custom_configs::opensearch::OpensearchConfig;
use crate::services::Service;
use loco_rs::app::AppContext;
use opensearch::http::transport::{SingleNodeConnectionPool, TransportBuilder};
use opensearch::http::Url;
use opensearch::OpenSearch;
use std::sync::{Arc, OnceLock};

pub type OpensearchClient = Arc<OpensearchClientInner>;

pub struct OpensearchClientInner {
    config: OpensearchConfig,
    opensearch: OpenSearch,
}

impl Service for OpensearchClientInner {
    async fn new(ctx: &AppContext) -> loco_rs::Result<Self> {
        let custom_config = CustomConfigInner::get_arc(ctx).await?;

        let internal_client = Self::get_client(&custom_config.opensearch).await?;

        Ok(Self {
            config: custom_config.opensearch.clone(),
            opensearch: internal_client,
        })
    }

    fn get_static_once() -> &'static OnceLock<Arc<Self>> {
        static INSTANCE: OnceLock<OpensearchClient> = OnceLock::new();

        &INSTANCE
    }
}

impl OpensearchClientInner {
    async fn get_client(config: &OpensearchConfig) -> AppResult<OpenSearch> {
        let url = Url::parse(config.get_url().as_str())?;
        let pool = SingleNodeConnectionPool::new(url);

        let mut transport = TransportBuilder::new(pool);
        if config.disable_proxy {
            transport = transport.disable_proxy();
        }
        let transport = transport.build()?;

        Ok(OpenSearch::new(transport))
    }
}
