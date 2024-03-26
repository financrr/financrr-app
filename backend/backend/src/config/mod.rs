use std::env;
use std::str::FromStr;

use tracing::log::LevelFilter;

use crate::CONFIG;

pub(crate) mod logger;
pub(crate) mod public;

#[derive(Debug, Clone)]
pub(crate) struct Config {
    pub(crate) address: String,
    pub(crate) database: DatabaseConfig,
    pub(crate) cache: RedisConfig,
    pub(crate) search: OpenSearchConfig,
    pub(crate) cors: CorsConfig,
    pub(crate) session: SessionConfig,
    pub(crate) rate_limiter: RateLimiterConfig,
}

#[derive(Debug, Clone)]
pub(crate) struct DatabaseConfig {
    pub(crate) host: String,
    pub(crate) port: u16,
    pub(crate) user: String,
    pub(crate) password: String,
    pub(crate) name: String,
    pub(crate) schema: String,
    pub(crate) min_connections: u32,
    pub(crate) max_connections: u32,
    pub(crate) sqlx_logging: bool,
    pub(crate) sqlx_log_level: LevelFilter,
}

#[derive(Debug, Clone)]
pub(crate) struct RedisConfig {
    pub(crate) host: String,
    pub(crate) port: u16,
}

#[derive(Debug, Clone)]
pub(crate) struct OpenSearchConfig {
    pub(crate) host: String,
    pub(crate) port: u16,
    pub(crate) username: String,
    pub(crate) password: String,
}

#[derive(Debug, Clone)]
pub(crate) struct SessionConfig {
    pub(crate) lifetime_hours: u64,
    pub(crate) limit: u64,
}

#[derive(Debug, Clone)]
pub(crate) struct RateLimiterConfig {
    pub(crate) limit: u64,
    pub(crate) duration_seconds: u64,
}

#[derive(Debug, Clone)]
pub(crate) struct CorsConfig {
    pub(crate) allowed_origins: Vec<String>,
    pub(crate) allow_any_origin: bool,
}

impl Config {
    pub(crate) fn get_database_url(&self) -> String {
        format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.database.user, self.database.password, self.database.host, self.database.port, self.database.name,
        )
    }

    pub(crate) fn load() -> Self {
        Self {
            address: get_env_or_error("ADDRESS"),
            database: DatabaseConfig::build_config(),
            cache: RedisConfig::build_config(),
            search: OpenSearchConfig::build_config(),
            cors: CorsConfig::build_config(),
            session: SessionConfig::build_config(),
            rate_limiter: RateLimiterConfig::build_config(),
        }
    }

    pub(crate) fn get_config<'a>() -> &'a Self {
        CONFIG.get().unwrap_or_else(|| {
            CONFIG.set(Self::load()).expect("Could not load and set config!");
            CONFIG.get().expect("Could not get config!")
        })
    }
}

impl DatabaseConfig {
    pub(crate) fn build_config() -> Self {
        Self {
            host: get_env_or_error("DATABASE_HOST"),
            port: get_env_or_error("DATABASE_PORT").parse::<u16>().expect("Could not parse DATABASE_PORT to u16!"),
            user: get_env_or_error("DATABASE_USER"),
            password: get_env_or_error("DATABASE_PASSWORD"),
            name: get_env_or_error("DATABASE_NAME"),
            schema: get_env_or_default("DATABASE_SCHEMA", "Public"),
            min_connections: get_env_or_default("DATABASE_MIN_CONNECTIONS", "5")
                .parse::<u32>()
                .expect("Could not parse DATABASE_MIN_CONNECTIONS to u32!"),
            max_connections: get_env_or_default("DATABASE_MAX_CONNECTIONS", "100")
                .parse::<u32>()
                .expect("Could not parse DATABASE_MAX_CONNECTIONS to u32!"),
            sqlx_logging: get_env_or_default("SQLX_LOGGING", "false")
                .parse::<bool>()
                .expect("Could not parse SQLX_LOGGING to bool!"),
            sqlx_log_level: LevelFilter::from_str(&get_env_or_default("SQLX_LOG_LEVEL", "off"))
                .expect("Could not parse SQLX_LOG_LEVEL to LevelFilter!"),
        }
    }
}

impl RedisConfig {
    pub(crate) fn build_config() -> Self {
        Self {
            host: get_env_or_error("REDIS_HOST"),
            port: get_env_or_error("REDIS_PORT").parse::<u16>().expect("Could not parse REDIS_PORT to u16!"),
        }
    }

    pub(crate) fn get_url(&self) -> String {
        format!("redis://{}:{}", self.host, self.port)
    }
}

impl OpenSearchConfig {
    pub(crate) fn build_config() -> Self {
        Self {
            host: get_env_or_error("SEARCH_HOST"),
            port: get_env_or_error("SEARCH_PORT").parse::<u16>().expect("Could not parse SEARCH_PORT to u16!"),
            username: get_env_or_error("SEARCH_USERNAME"),
            password: get_env_or_error("SEARCH_PASSWORD"),
        }
    }

    pub(crate) fn get_url(&self) -> String {
        format!("http://{}:{}", self.host, self.port)
    }
}

impl CorsConfig {
    pub(crate) fn build_config() -> Self {
        let allowed_origins =
            get_env_or_default("CORS_ALLOWED_ORIGINS", "").split(',').map(|s| s.to_string()).collect();

        Self {
            allowed_origins,
            allow_any_origin: get_env_or_default("CORS_ALLOW_ANY_ORIGIN", "false").parse::<bool>().unwrap_or(false),
        }
    }
}

impl SessionConfig {
    pub(crate) fn build_config() -> Self {
        Self {
            lifetime_hours: get_env_or_default("SESSION_LIFETIME_HOURS", "168")
                .parse::<u64>()
                .expect("Could not parse SESSION_LIFETIME_HOURS to u64!"),
            limit: get_env_or_default("SESSION_LIMIT", "25")
                .parse::<u64>()
                .expect("Could not parse SESSION_LIMIT to u64!"),
        }
    }
}

impl RateLimiterConfig {
    pub(crate) fn build_config() -> Self {
        Self {
            limit: get_env_or_default("RATE_LIMITER_LIMIT", "5000")
                .parse::<u64>()
                .expect("Could not parse RATE_LIMITER_LIMIT to u64!"),
            duration_seconds: get_env_or_default("RATE_LIMITER_DURATION_SECONDS", "3600")
                .parse::<u64>()
                .expect("Could not parse RATE_LIMITER_DURATION to u64!"),
        }
    }
}

pub(crate) fn get_env_or_error(key: &str) -> String {
    env::var(key).unwrap_or_else(|_| panic!("'{}' env variable must be set!", key))
}

pub(crate) fn get_env_or_default(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}
