use crate::models::_entities::external_bank_institutions::Column;
use crate::models::external_bank_institutions::ExternalBankInstitutions;
use crate::opensearch::external_bank_institutions::IndexableExternalBankInstitution;
use crate::services::opensearch::client::{OpensearchClientInner, OpensearchIndex};
use crate::services::Service;
use async_trait::async_trait;
use loco_rs::prelude::{AppContext, BackgroundWorker};
use opensearch::http::request::JsonBody;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use serde_json::json;
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
            let mut body: Vec<JsonBody<_>> = Vec::with_capacity(institutions.len());

            for institution in institutions {
                let institution = IndexableExternalBankInstitution::from(institution);

                body.push(json!({"index": {"_id": institution.id}}).into());
                body.push(json!(institution).into());
            }

            opensearch
                .bulk_insert(OpensearchIndex::EXTERNAL_BANK_INSTITUTIONS, body)
                .await?;
        }

        info!("Finished indexing external institutions");

        Ok(())
    }
}
