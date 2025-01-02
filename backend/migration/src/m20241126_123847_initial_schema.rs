use crate::sea_orm::{DbBackend, Statement, TransactionError, TransactionTrait};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

const SCHEMA: &str = include_str!("initial_schema.sql");

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let statements = get_statements();
        let conn = manager.get_connection();

        conn.transaction::<_, _, DbErr>(|txn| {
            Box::pin(async move {
                for stmt in statements {
                    txn.execute(stmt).await?;
                }

                Ok(())
            })
        })
        .await
        .map_err(|e| match e {
            TransactionError::Connection(err) => err,
            TransactionError::Transaction(e) => DbErr::Custom(e.to_string()),
        })
    }

    async fn down(&self, _: &SchemaManager) -> Result<(), DbErr> {
        panic!("Cannot migrate down for the initial schema migration!")
    }
}

fn get_statements() -> Vec<Statement> {
    SCHEMA
        .split(";")
        .map(|s| Statement::from_string(DbBackend::Postgres, s))
        .collect()
}
