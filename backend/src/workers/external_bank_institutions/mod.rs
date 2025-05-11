use crate::workers::external_bank_institutions::clean_up::CleanUpExternalInstitutions;
use crate::workers::external_bank_institutions::delete_from_index::DeleteInstitutionsFromIndex;
use crate::workers::external_bank_institutions::index_external_institutions::IndexExternalInstitutionsWorker;
use crate::workers::external_bank_institutions::sync_go_cardless_institutions::SyncGoCardlessInstitutionsWorker;
use loco_rs::prelude::{AppContext, BackgroundWorker, Queue};

mod clean_up;
pub(super) mod delete_from_index;
mod index_external_institutions;
pub mod sync_go_cardless_institutions;

pub async fn connect_worker(ctx: &AppContext, queue: &Queue) -> loco_rs::Result<()> {
    queue.register(IndexExternalInstitutionsWorker::build(ctx)).await?;
    queue.register(CleanUpExternalInstitutions::build(ctx)).await?;
    queue.register(DeleteInstitutionsFromIndex::build(ctx)).await?;
    queue.register(SyncGoCardlessInstitutionsWorker::build(ctx)).await?;

    Ok(())
}
