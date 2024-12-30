use crate::api::error::validation::ValidationCode;
use crate::wrapper::entity::transaction::template::TransactionTemplate;
use crate::wrapper::types::phantom::{Identifiable, Phantom};
use tokio::runtime::Handle;
use validator::ValidationError;

pub(crate) fn assert_template_exists(template_id: &Phantom<TransactionTemplate>) -> Result<(), ValidationError> {
    Handle::current().block_on(async {
        if TransactionTemplate::find_by_id(template_id.get_id()).await.is_err() {
            ValidationCode::ENTITY_NOT_FOUND.into()
        } else {
            Ok(())
        }
    })
}
