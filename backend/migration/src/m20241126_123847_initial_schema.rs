use crate::sea_orm::{DbBackend, Statement};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

const SCHEMA: &str = include_str!("initial_schema.sql");

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let statements = get_statements();
        for stmt in statements {
            manager.get_connection().execute(stmt).await?;
        }

        Ok(())
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
