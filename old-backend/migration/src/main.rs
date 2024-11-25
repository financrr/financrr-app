use dotenvy::dotenv;
use sea_orm_migration::prelude::*;

#[tokio::main]
async fn main() {
    dotenv().ok();
    cli::run_cli(migration::Migrator).await;
}
