use crate::error::app_error::{AppError, AppResult};
use crate::opensearch::indices::OpensearchIndex;
use crate::opensearch::migration_system::OpensearchMigration;
use crate::services::opensearch::client::OpensearchClient;
use crate::utils::type_name::type_name_only;
use async_trait::async_trait;
use loco_rs::app::AppContext;
use opensearch::indices::{IndicesCreateParts, IndicesDeleteParts};

pub struct Version02022025;

#[async_trait]
impl OpensearchMigration for Version02022025 {
    fn name(&self) -> String {
        type_name_only::<Self>().to_string()
    }

    async fn up(&self, _ctx: &AppContext, client: &OpensearchClient) -> AppResult<()> {
        let res = client
            .opensearch
            .indices()
            .create(IndicesCreateParts::Index(
                OpensearchIndex::EXTERNAL_BANK_INSTITUTIONS.name,
            ))
            .body(OpensearchIndex::EXTERNAL_BANK_INSTITUTIONS.get_mapping())
            .send()
            .await?;

        if !res.status_code().is_success() {
            return Err(AppError::OpensearchError(res.text().await?));
        }

        Ok(())
    }

    async fn down(&self, _ctx: &AppContext, client: &OpensearchClient) -> AppResult<()> {
        client
            .opensearch
            .indices()
            .delete(IndicesDeleteParts::Index(&[
                OpensearchIndex::EXTERNAL_BANK_INSTITUTIONS.name,
            ]))
            .send()
            .await?;

        Ok(())
    }
}
