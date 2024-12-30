use crate::api::error::validation::ValidationCode;
use crate::wrapper::entity::transaction::dto::TransactionDTO;
use validator::ValidationError;

// TODO remove
#[allow(dead_code)]
pub(crate) fn validate_source_and_destination(dto: &TransactionDTO) -> Result<(), ValidationError> {
    if dto.source_id.is_none() && dto.destination_id.is_none() {
        return ValidationCode::SOURCE_AND_DESTINATION_MISSING.into();
    }

    Ok(())
}
