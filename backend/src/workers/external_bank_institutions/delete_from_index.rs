use crate::opensearch::indices::OpensearchIndex;
use crate::services::opensearch::client::OpensearchClientInner;
use crate::services::Service;
use async_trait::async_trait;
use loco_rs::prelude::{AppContext, BackgroundWorker};
use serde::{Deserialize, Serialize};

pub(super) struct DeleteInstitutionFromIndex {
    ctx: AppContext,
}

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct DeleteInstitutionFromIndexArgs {
    pub id: i64,
}

#[async_trait]
impl BackgroundWorker<DeleteInstitutionFromIndexArgs> for DeleteInstitutionFromIndex {
    fn build(ctx: &AppContext) -> Self {
        Self { ctx: ctx.clone() }
    }

    async fn perform(&self, args: DeleteInstitutionFromIndexArgs) -> loco_rs::Result<()> {
        let client = OpensearchClientInner::get_arc(&self.ctx).await?;

        client
            .delete(OpensearchIndex::EXTERNAL_BANK_INSTITUTIONS.name, args.id)
            .await?;

        Ok(())
    }
}
