use crate::workers::external_bank_institutions::delete_from_index::{
    DeleteInstitutionFromIndex, DeleteInstitutionFromIndexArgs,
};
use async_trait::async_trait;
use loco_rs::app::AppContext;
use loco_rs::prelude::BackgroundWorker;
use serde::{Deserialize, Serialize};

pub struct ExternalBankInstitutionDeleted {
    ctx: AppContext,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalBankInstitutionDeletedArgs {
    pub id: i64,
}

#[async_trait]
impl BackgroundWorker<ExternalBankInstitutionDeletedArgs> for ExternalBankInstitutionDeleted {
    fn build(ctx: &AppContext) -> Self {
        Self { ctx: ctx.clone() }
    }

    async fn perform(&self, args: ExternalBankInstitutionDeletedArgs) -> loco_rs::Result<()> {
        _ = DeleteInstitutionFromIndex::perform_later(&self.ctx, DeleteInstitutionFromIndexArgs { id: args.id }).await;

        Ok(())
    }
}
