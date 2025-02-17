use crate::error::app_error::AppResult;
use crate::services::opensearch::client::OpensearchClient;
use async_trait::async_trait;
use loco_rs::prelude::AppContext;

pub mod migrator;

#[async_trait]
pub trait OpensearchMigration: Send {
    fn name(&self) -> String;

    async fn up(&self, ctx: &AppContext, client: &OpensearchClient) -> AppResult<()>;

    async fn down(&self, ctx: &AppContext, client: &OpensearchClient) -> AppResult<()>;
}
