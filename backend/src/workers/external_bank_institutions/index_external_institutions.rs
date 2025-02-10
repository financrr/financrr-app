use crate::models::_entities::external_bank_institutions::Column;
use crate::models::external_bank_institutions::ExternalBankInstitutions;
use crate::opensearch::indices::OpensearchIndex;
use crate::opensearch::models::external_bank_institutions::IndexableExternalBankInstitution;
use crate::services::opensearch::client::OpensearchClientInner;
use crate::services::Service;
use async_trait::async_trait;
use loco_rs::prelude::{AppContext, BackgroundWorker};
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use tracing::info;

pub struct IndexExternalInstitutionsWorker {
    ctx: AppContext,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexExternalInstitutionsWorkerArgs {
    pub provider: String,
}

#[async_trait]
impl BackgroundWorker<IndexExternalInstitutionsWorkerArgs> for IndexExternalInstitutionsWorker {
    fn build(ctx: &AppContext) -> Self {
        Self { ctx: ctx.clone() }
    }

    async fn perform(&self, args: IndexExternalInstitutionsWorkerArgs) -> loco_rs::Result<()> {
        info!("Indexing {} external institutions.", args.provider);

        let opensearch = OpensearchClientInner::get_arc(&self.ctx).await?;

        const PAGE_SIZE: u64 = 250;
        let mut paginator = ExternalBankInstitutions::find()
            .filter(Column::Provider.eq(args.provider.as_str()))
            .paginate(&self.ctx.db, PAGE_SIZE);

        while let Some(institutions) = paginator.fetch_and_next().await? {
            let institutions: Vec<(i64, IndexableExternalBankInstitution)> = institutions
                .into_iter()
                .map(|i| (i.id, IndexableExternalBankInstitution::from(i)))
                .collect();

            opensearch
                .bulk_insert(OpensearchIndex::EXTERNAL_BANK_INSTITUTIONS.name, institutions)
                .await?;
        }

        info!("Finished indexing external institutions");

        Ok(())
    }
}
