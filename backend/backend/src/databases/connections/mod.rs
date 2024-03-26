use std::sync::OnceLock;

use ::redis::Client;
use sea_orm::DatabaseConnection;
use tracing::info;

use search::OpenSearch;

pub(crate) mod psql;
pub(crate) mod redis;
pub(crate) mod search;

pub(crate) static DB_CONN: OnceLock<DatabaseConnection> = OnceLock::new();
pub(crate) static REDIS_CONN: OnceLock<Client> = OnceLock::new();
pub(crate) static SEARCH_CONN: OnceLock<OpenSearch> = OnceLock::new();

pub(crate) async fn init_data_sources() {
    info!("\t[*] Establishing database connection...");
    DB_CONN.set(psql::establish_database_connection().await).expect("Could not set database connection!");

    info!("\t[*] Establishing redis connection...");
    REDIS_CONN.set(redis::create_redis_client().await).expect("Could not set redis connection!");

    info!("\t[*] Establishing search connection...");
    SEARCH_CONN.set(search::create_open_search_client().await).expect("Could not set search connection!");
}
