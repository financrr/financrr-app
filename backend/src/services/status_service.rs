use crate::services::Service;
use crate::services::opensearch::client::{OpensearchClient, OpensearchClientInner};
use crate::views::status::{HealthReport, HealthResponse, HealthStatus, StatusComponents};
use bytes::Bytes;
use loco_rs::app::AppContext;
use std::path::PathBuf;
use std::sync::{Arc, LazyLock, OnceLock};
use tokio::join;

pub type StatusService = Arc<StatusServiceInner>;

pub const TEST_CACHE_KEY: &str = "test_cache_key";
pub const TEST_CACHE_VALUE: &str = "test_cache_value";

pub static TEST_STORAGE_PATH: LazyLock<PathBuf> = LazyLock::new(|| PathBuf::from("test_storage_path"));
pub const TEST_STORAGE_CONTENT_STR: &str = "test_storage_content";
pub static TEST_STORAGE_CONTENT_BYTES: LazyLock<Bytes> = LazyLock::new(|| Bytes::from(TEST_STORAGE_CONTENT_STR));

pub struct StatusServiceInner {
    ctx: AppContext,
    opensearch_client: OpensearchClient,
}

impl Service for StatusServiceInner {
    async fn new(ctx: &AppContext) -> loco_rs::Result<Self> {
        Ok(Self {
            ctx: ctx.clone(),
            opensearch_client: OpensearchClientInner::get_arc(ctx).await?,
        })
    }

    fn get_static_once() -> &'static OnceLock<Arc<Self>> {
        static INSTANCE: OnceLock<Arc<StatusServiceInner>> = OnceLock::new();

        &INSTANCE
    }
}

impl StatusServiceInner {
    pub async fn get_db_health(&self) -> HealthReport {
        let db = &self.ctx.db;
        let mut failed_components = Vec::new();

        if db.ping().await.is_err() {
            failed_components.push(StatusComponents::Database);
        }

        let status = if failed_components.is_empty() {
            HealthStatus::Healthy
        } else {
            HealthStatus::Unhealthy
        };

        HealthReport {
            status,
            failed_components: if failed_components.is_empty() {
                None
            } else {
                Some(failed_components)
            },
        }
    }

    pub async fn get_cache_health(&self) -> HealthReport {
        let cache = &self.ctx.cache;
        let mut failed_components = Vec::new();

        if cache.insert(TEST_CACHE_KEY, TEST_CACHE_VALUE).await.is_err() {
            failed_components.push(StatusComponents::CacheInsertion);
        }

        match cache.get(TEST_CACHE_KEY).await {
            Ok(Some(value)) if value.eq(TEST_CACHE_VALUE) => true,
            _ => {
                failed_components.push(StatusComponents::CacheRetrieval);
                false
            }
        };

        if cache.remove(TEST_CACHE_KEY).await.is_err() {
            failed_components.push(StatusComponents::CacheDeletion);
        }

        let status = if failed_components.is_empty() {
            HealthStatus::Healthy
        } else {
            HealthStatus::Unhealthy
        };

        HealthReport {
            status,
            failed_components: if failed_components.is_empty() {
                None
            } else {
                Some(failed_components)
            },
        }
    }

    pub async fn get_storage_health(&self) -> HealthReport {
        let storage = &self.ctx.storage;
        let mut failed_components = Vec::new();

        if storage
            .upload(TEST_STORAGE_PATH.as_path(), &TEST_STORAGE_CONTENT_BYTES)
            .await
            .is_err()
        {
            failed_components.push(StatusComponents::StorageInsertion);
        }

        match storage.download::<String>(TEST_STORAGE_PATH.as_path()).await {
            Ok(content) if content.eq(TEST_STORAGE_CONTENT_STR) => true,
            _ => {
                failed_components.push(StatusComponents::StorageRetrieval);
                false
            }
        };

        let delete = storage.delete(&TEST_STORAGE_PATH).await.is_ok();
        if !delete {
            failed_components.push(StatusComponents::StorageDeletion);
        }

        let status = if failed_components.is_empty() {
            HealthStatus::Healthy
        } else {
            HealthStatus::Unhealthy
        };

        HealthReport {
            status,
            failed_components: if failed_components.is_empty() {
                None
            } else {
                Some(failed_components)
            },
        }
    }

    pub async fn get_opensearch_health(&self) -> HealthReport {
        let mut failed_components = vec![];

        if !self.opensearch_client.is_healthy().await {
            failed_components.push(StatusComponents::Opensearch);
        }

        let health_status = HealthStatus::from(failed_components.is_empty());

        HealthReport {
            status: health_status,
            failed_components: if failed_components.is_empty() {
                None
            } else {
                Some(failed_components)
            },
        }
    }

    pub async fn get_complete_health_response(&self) -> HealthResponse {
        let (database_status, cache_status, storage_status, opensearch_status) = join!(
            self.get_db_health(),
            self.get_cache_health(),
            self.get_storage_health(),
            self.get_opensearch_health()
        );

        let status = if matches!(database_status.status, HealthStatus::Healthy)
            && matches!(cache_status.status, HealthStatus::Healthy)
            && matches!(storage_status.status, HealthStatus::Healthy)
            && matches!(opensearch_status.status, HealthStatus::Healthy)
        {
            HealthStatus::Healthy
        } else {
            HealthStatus::Unhealthy
        };

        HealthResponse {
            status,
            database_status,
            cache_status,
            storage_status,
            opensearch_status,
        }
    }
}
