use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use crate::config::Config;
use crate::databases::connections::DB_CONN;

pub(crate) fn get_database_connection<'a>() -> &'a DatabaseConnection {
    DB_CONN.get().expect("Could not get database connection!")
}

pub(crate) async fn establish_database_connection() -> DatabaseConnection {
    let db = Database::connect(get_connection_options()).await.expect("Could not open database connection!");
    assert!(db.ping().await.is_ok());
    db
}

fn get_connection_options() -> ConnectOptions {
    let config = Config::get_config();
    let mut opt = ConnectOptions::new(config.get_database_url());
    opt.max_connections(config.database.max_connections)
        .min_connections(config.database.min_connections)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(config.database.sqlx_logging)
        .sqlx_logging_level(config.database.sqlx_log_level)
        .set_schema_search_path(&config.database.schema)
        .clone()
}
