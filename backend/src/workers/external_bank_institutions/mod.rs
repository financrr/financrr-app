use crate::workers::external_bank_institutions::add_to_index::AddInstitutionToIndex;
use crate::workers::external_bank_institutions::crud::deleted::ExternalBankInstitutionDeleted;
use crate::workers::external_bank_institutions::crud::insert::ExternalBankInstitutionInserted;
use crate::workers::external_bank_institutions::crud::update::ExternalBankInstitutionUpdated;
use crate::workers::external_bank_institutions::delete_from_index::DeleteInstitutionFromIndex;
use loco_rs::prelude::{AppContext, BackgroundWorker, Queue};

pub(super) mod add_to_index;
pub mod crud;
pub(super) mod delete_from_index;

pub async fn connect_worker(ctx: &AppContext, queue: &Queue) -> loco_rs::Result<()> {
    //crud
    queue.register(ExternalBankInstitutionDeleted::build(ctx)).await?;
    queue.register(ExternalBankInstitutionInserted::build(ctx)).await?;
    queue.register(ExternalBankInstitutionUpdated::build(ctx)).await?;

    queue.register(AddInstitutionToIndex::build(ctx)).await?;
    queue.register(DeleteInstitutionFromIndex::build(ctx)).await?;

    Ok(())
}
