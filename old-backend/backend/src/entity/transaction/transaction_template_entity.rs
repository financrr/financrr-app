use crate::api::error::api::ApiError;
use crate::entity::db_model::transaction_template::Model;
use crate::entity::transaction::transaction_type::TransactionType;
use crate::snowflake::snowflake_type::Snowflake;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash, ToSchema, Serialize)]
pub(crate) struct TransactionTemplate {
    pub(crate) id: Snowflake,
    pub(crate) transaction_type: TransactionType,
    pub(crate) amount: i64,
    pub(crate) currency: Snowflake,
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) budget: Option<Snowflake>,
}

impl TryFrom<Model> for TransactionTemplate {
    type Error = ApiError;


    fn try_from(value: Model) -> Result<Self, Self::Error> {
        Ok(TransactionTemplate {
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
