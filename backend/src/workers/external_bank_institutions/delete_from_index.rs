use crate::opensearch::indices::OpensearchIndex;
use crate::services::opensearch::client::OpensearchClientInner;
use crate::services::Service;
use async_trait::async_trait;
use loco_rs::prelude::{AppContext, BackgroundWorker};
use serde::{Deserialize, Serialize};

pub(super) struct DeleteInstitutionsFromIndex {
    ctx: AppContext,
}

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct DeleteInstitutionFromIndexArgs {
    pub ids: Vec<i64>,
}

#[async_trait]
impl BackgroundWorker<DeleteInstitutionFromIndexArgs> for DeleteInstitutionsFromIndex {
    fn build(ctx: &AppContext) -> Self {
        Self { ctx: ctx.clone() }
    }

    async fn perform(&self, args: DeleteInstitutionFromIndexArgs) -> loco_rs::Result<()> {
        let client = OpensearchClientInner::get_arc(&self.ctx).await?;

        // TODO only bulk remove 500 at once at max
        client
            .bulk_delete(OpensearchIndex::EXTERNAL_BANK_INSTITUTIONS.name, args.ids)
            .await?;

        Ok(())
    }
}
