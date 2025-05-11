use crate::services::Service;
use crate::services::bank_linking_data::BankLinkingDataInner;
use crate::workers::external_bank_institutions::sync_go_cardless_institutions::{
    SyncGoCardlessInstitutionsWorker, WorkerArgs,
};
use loco_rs::prelude::*;
use tracing::info;

pub struct SyncInstitutions;

#[async_trait]
impl Task for SyncInstitutions {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "SyncInstitutions".to_string(),
            detail: "Syncs external Institutions with our DB.".to_string(),
        }
    }

    async fn run(&self, ctx: &AppContext, _vars: &task::Vars) -> Result<()> {
        let service = BankLinkingDataInner::get_arc(ctx).await?;

        if service.get_go_cardless_client().is_some() {
            info!("Syncing GoCardless institutions");
            SyncGoCardlessInstitutionsWorker::perform_later(ctx, WorkerArgs).await?;
        }

        Ok(())
    }
}
