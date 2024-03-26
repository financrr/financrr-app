use crate::api::error::api::ApiError;
use crate::config::Config;
use crate::databases::connections::REDIS_CONN;
use redis::aio::MultiplexedConnection;
use redis::Client;

pub(crate) async fn get_redis_connection() -> Result<MultiplexedConnection, ApiError> {
    let pool = REDIS_CONN.get().expect("Could not get redis pool!");
    pool.get_multiplexed_tokio_connection().await.map_err(ApiError::from)
}

pub(crate) async fn create_redis_client() -> Client {
    let config = Config::get_config();
    Client::open(config.cache.get_url()).expect("Could not open redis connection!")
}
