use crate::api::error::api::ApiError;
use crate::entity::db_model::transaction::Model;
use crate::entity::transaction::transaction_type::TransactionType;
use crate::snowflake::snowflake_type::Snowflake;

pub(crate) struct Transaction {
    pub(crate) id: Snowflake,
    pub(crate) transaction_type: TransactionType,
    pub(crate) amount: i64,
    pub(crate) currency: Snowflake,
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) budget: Option<Snowflake>,
}

impl TryFrom<Model> for Transaction {
    type Error = ApiError;

    fn try_from(value: Model) -> Result<Self, Self::Error> {
        Ok(Transaction {
            id: Snowflake::new(value.id),
            transaction_type: TransactionType::try_from((value.source, value.destination))?,
            amount: value.amount,
            currency: Snowflake::new(value.currency),
            name: value.name,
            description: value.description,
            budget: value.budget.map(Snowflake::new),
        })
    }
}
