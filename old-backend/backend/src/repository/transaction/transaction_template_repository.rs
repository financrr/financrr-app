use sea_orm::ConnectionTrait;
use crate::entity::db_model::transaction_template::Model;
use crate::entity::transaction::transaction_template_entity::TransactionTemplate;
use crate::repository::traits::Repository;

pub(crate) struct TransactionTemplateRepository {
    pub(crate) conn: &'static dyn ConnectionTrait,
}

impl Repository<Model, TransactionTemplate> for TransactionTemplateRepository {
    fn new(conn: &impl ConnectionTrait) -> Self {
        TransactionTemplateRepository {
            conn,
        }
    }
}
