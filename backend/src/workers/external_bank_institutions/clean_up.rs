use crate::bank_account_linking::constants::GO_CARDLESS_PROVIDER;
use crate::models::external_bank_institutions::ActiveModel;
use crate::workers::external_bank_institutions::index_external_institutions::{
    IndexExternalInstitutionsWorker, IndexExternalInstitutionsWorkerArgs,
};
use async_trait::async_trait;
use loco_rs::prelude::{AppContext, BackgroundWorker};
use serde::{Deserialize, Serialize};
use tracing::info;

pub struct CleanUpExternalInstitutions {
    ctx: AppContext,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CleanUpExternalInstitutionsArgs {
    pub external_ids: Vec<String>,
    pub provider: String,
}

#[async_trait]
impl BackgroundWorker<CleanUpExternalInstitutionsArgs> for CleanUpExternalInstitutions {
    fn build(ctx: &AppContext) -> Self {
        Self { ctx: ctx.clone() }
    }

    async fn perform(&self, args: CleanUpExternalInstitutionsArgs) -> loco_rs::Result<()> {
        info!("Cleaning up external institutions for {}", args.provider);

        ActiveModel::delete_unknown_institutions(&self.ctx.db, args.external_ids, args.provider).await?;

        // Index all the inserted entities into opensearch
        IndexExternalInstitutionsWorker::perform_later(
            &self.ctx,
            IndexExternalInstitutionsWorkerArgs {
                provider: GO_CARDLESS_PROVIDER.to_string(),
            },
        )
        .await?;

        info!("Finished clean up external institutions");
        Ok(())
    }
}
