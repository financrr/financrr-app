use crate::api::error::validation::ValidationCode;
use crate::wrapper::entity::budget::Budget;
use crate::wrapper::types::phantom::Phantom;
use tokio::runtime::Handle;
use validator::ValidationError;

pub(crate) fn budget_exists(budget: &&Phantom<Budget>) -> Result<(), ValidationError> {
    Handle::current().block_on(async {
        if !Budget::exists(budget.get_id()).await? {
            return ValidationCode::ENTITY_NOT_FOUND.into();
        }

        Ok(())
    })
}
