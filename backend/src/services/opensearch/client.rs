use crate::error::app_error::{AppError, AppResult};
use crate::services::custom_configs::base::CustomConfigInner;
use crate::services::custom_configs::opensearch::OpensearchConfig;
use crate::services::Service;
use crate::views::pagination::{Pager, PagerMeta};
use loco_rs::app::AppContext;
use opensearch::auth::Credentials;
use opensearch::cert::CertificateValidation;
use opensearch::cluster::ClusterHealthParts;
use opensearch::http::transport::{SingleNodeConnectionPool, TransportBuilder};
use opensearch::http::Url;
use opensearch::{BulkOperation, BulkParts, OpenSearch, SearchParts};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{json, Value};
use std::sync::{Arc, OnceLock};
use tracing::warn;

pub type OpensearchClient = Arc<OpensearchClientInner>;

#[derive(Debug)]
pub struct OpensearchClientInner {
    pub opensearch: Arc<OpenSearch>,
}

impl Service for OpensearchClientInner {
    async fn new(ctx: &AppContext) -> loco_rs::Result<Self> {
        let custom_config = CustomConfigInner::get_arc(ctx).await?;

        let internal_client = Self::get_client(&custom_config.opensearch).await?;

        Ok(Self {
            opensearch: Arc::new(internal_client),
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
        if config.disable_cert_validation {
            transport = transport.cert_validation(CertificateValidation::None);
        }
        transport = transport.auth(Credentials::Basic(config.username.clone(), config.password.clone()));

        let transport = transport.build()?;

        Ok(OpenSearch::new(transport))
    }

    pub async fn is_healthy(&self) -> bool {
        let rs = self.is_healthy_result().await;
        if let Err(e) = &rs {
            warn!("Opensearch is unhealthy: {}", e)
        }

        rs.unwrap_or(false)
    }

    async fn is_healthy_result(&self) -> AppResult<bool> {
        let res = self
            .opensearch
            .cluster()
            .health(ClusterHealthParts::None)
            .send()
            .await?;
        if res.status_code().as_u16() > 299 {
            return Ok(false);
        }

        let res: Value = res.json().await?;

        match res.get("status") {
            None => Ok(false),
            Some(status) => {
                if let Some(str) = status.as_str() {
                    if !str.eq("red") {
                        return Ok(true);
                    }
                }

                Ok(false)
            }
        }
    }

    pub fn get_inner_client(&self) -> Arc<OpenSearch> {
        self.opensearch.clone()
    }

    pub async fn bulk_insert<T: Serialize>(&self, index: &str, docs: Vec<(i64, T)>) -> AppResult<()> {
        let mut body: Vec<BulkOperation<Value>> = Vec::with_capacity(docs.len());

        for (id, doc) in docs {
            body.push(BulkOperation::index(json!(doc)).id(id.to_string()).into())
        }

        let res = self.opensearch.bulk(BulkParts::Index(index)).body(body).send().await?;
        if !res.status_code().is_success() {
            return Err(AppError::OpensearchError(res.text().await?));
        }

        Ok(())
    }

    pub async fn bulk_delete(&self, index: &str, ids: Vec<i64>) -> AppResult<()> {
        let mut body: Vec<BulkOperation<Value>> = Vec::with_capacity(ids.len());
        for id in ids {
            body.push(BulkOperation::delete(id.to_string()).into())
        }

        let res = self.opensearch.bulk(BulkParts::Index(index)).body(body).send().await?;

        if !res.status_code().is_success() {
            return Err(AppError::OpensearchError(res.text().await?));
        }

        Ok(())
    }

    pub async fn search<T>(&self, index: &str, body: Value, page: u64, page_size: u64) -> AppResult<Pager<T>>
    where
        T: DeserializeOwned,
    {
        let value = self.search_custom(index, body).await?;

        let default_hits = vec![];
        let hits = value["hits"]["hits"].as_array().unwrap_or(&default_hits);
        let total = value["hits"]["total"]["value"].as_u64().unwrap_or(0);

        let results: Vec<T> = hits
            .iter()
            .map(|hit| serde_json::from_value(hit["_source"].clone()).expect("Unexpected data in index"))
            .collect();

        let pager = Pager {
            results,
            info: PagerMeta {
                page,
                page_size,
                total_pages: (total as f64 / page_size as f64).ceil() as u64,
                total,
            },
        };

        Ok(pager)
    }

    pub async fn search_custom(&self, index: &str, body: Value) -> AppResult<Value> {
        let res = self
            .opensearch
            .search(SearchParts::Index(&[index]))
            .body(body)
            .send()
            .await?;

        if !res.status_code().is_success() {
            return Err(AppError::OpensearchError(res.text().await?));
        }

        Ok(res.json().await?)
    }
}
