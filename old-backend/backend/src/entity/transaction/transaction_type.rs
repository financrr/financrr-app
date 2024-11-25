use crate::snowflake::snowflake_type::Snowflake;
use serde::Serialize;
use utoipa::ToSchema;
use crate::api::error::api::ApiError;

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash, ToSchema, Serialize)]
pub(crate) enum TransactionType {
    Withdrawal { source: Snowflake },
    Deposit { destination: Snowflake },
    Transfer { source: Snowflake, destination: Snowflake },
}

impl TryFrom<(Option<i64>, Option<i64>)> for TransactionType {
    type Error = ApiError;

    fn try_from(value: (Option<i64>, Option<i64>)) -> Result<Self, Self::Error> {
        match value {
            (Some(source), None) => Ok(TransactionType::Withdrawal { source: Snowflake::new(source) }),
            (None, Some(destination)) => Ok(TransactionType::Deposit { destination: Snowflake::new(destination) }),
            (Some(source), Some(destination)) => Ok(TransactionType::Transfer { source: Snowflake::new(source), destination: Snowflake::new(destination) }),
            (None, None) => Err(ApiError::InvalidTransactionType()),
        }
    }
}
