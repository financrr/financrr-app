use crate::models::_entities::instances;
use crate::services::Service;
use crate::utils::datetime::get_epoch_millis;
use loco_rs::app::AppContext;
use loco_rs::prelude::Result;
use loco_rs::Error;
use sea_orm::{DatabaseConnection, IntoActiveModel};
use std::env::VarError;
use std::num::ParseIntError;
use std::process::exit;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use thiserror::Error;
use tokio::{spawn, time};
use tracing::error;

pub const SNOWFLAKE_EPOCH: i64 = 1_705_247_483_000;

pub const NODE_ID_BITS: u8 = 10;
pub const SEQUENCE_BITS: u8 = 12;

pub const TIMESTAMP_SHIFT: u8 = NODE_ID_BITS + SEQUENCE_BITS;

pub const MAX_NODE_ID: u64 = (1 << NODE_ID_BITS) - 1;
pub const MAX_SEQUENCE: u64 = (1 << SEQUENCE_BITS) - 1;

pub const SNOWFLAKE_HEARTBEAT_INTERVAL_SECONDS: u64 = 10;

pub type SnowflakeGenerator = Arc<SnowflakeGeneratorInner>;

#[derive(Debug)]
pub struct SnowflakeGeneratorInner {
    node_id: u64,
    last_timestamp: AtomicU64,
    sequence: AtomicU64,
}

impl Service for SnowflakeGeneratorInner {
    async fn new(ctx: &AppContext) -> Result<Self> {
        let instance = {
            let node_id = instances::Model::find_next_node_id(&ctx.db).await?;

            if node_id as u64 > MAX_NODE_ID {
                error!("Node ID is too large. Which means the maximum number of instancs has been started.");
                return Err(Error::Any(SnowflakeGeneratorError::NodeIdTooLarge.into()));
            }

            instances::Model::create_new_instance(&ctx.db, node_id).await?
        };

        let generator = Self::with_node_id(instance.node_id as u64);
        generator.start_heartbeat(ctx.db.clone());

        Ok(generator)
    }
}

impl SnowflakeGeneratorInner {
    pub fn with_node_id(node_id: u64) -> Self {
        Self {
            node_id,
            last_timestamp: AtomicU64::new(0),
            sequence: AtomicU64::new(0),
        }
    }

    fn start_heartbeat(&self, db: DatabaseConnection) {
        let node_id = self.node_id as i16;
        spawn(async move {
            let mut interval = time::interval(time::Duration::from_secs(SNOWFLAKE_HEARTBEAT_INTERVAL_SECONDS));
            loop {
                interval.tick().await;
                match instances::Model::find_by_node_id(&db, node_id).await {
                    Ok(instance) => {
                        let active_model = instance.into_active_model();
                        match active_model.update_heartbeat(&db).await {
                            Err(e) => {
                                error!("Failed to update heartbeat: {}", e);
                                exit(1);
                            }
                            _ => {}
                        }
                    }
                    Err(err) => {
                        error!(
                            "Failed to get instance by current node id: {} | Error: {}",
                            node_id, err
                        );
                        exit(1);
                    }
                }
            }
        });
    }

    pub fn next_id(&self) -> Result<i64> {
        let mut current_timestamp = self.timestamp();
        let last_timestamp = self.last_timestamp.load(Ordering::SeqCst);

        if current_timestamp < last_timestamp {
            return Err(Error::Any(SnowflakeGeneratorError::InvalidSystemClock.into()));
        }

        let mut sequence = self.sequence.load(Ordering::SeqCst);

        if current_timestamp == last_timestamp {
            sequence = (sequence + 1) & MAX_SEQUENCE;
            if sequence == 0 {
                current_timestamp = self.wait_for_next_millis(current_timestamp, last_timestamp);
            }
        } else {
            sequence = 0;
        }

        self.last_timestamp.store(current_timestamp, Ordering::SeqCst);
        self.sequence.store(sequence, Ordering::SeqCst);

        Ok(((current_timestamp << (NODE_ID_BITS + SEQUENCE_BITS)) | (self.node_id << SEQUENCE_BITS) | sequence) as i64)
    }

    fn timestamp(&self) -> u64 {
        get_epoch_millis() - (SNOWFLAKE_EPOCH as u64)
    }

    fn wait_for_next_millis(&self, mut current_timestamp: u64, last_timestamp: u64) -> u64 {
        while current_timestamp == last_timestamp {
            current_timestamp = self.timestamp();
        }

        current_timestamp
    }
}

#[derive(Error, Debug, Clone, Eq, PartialEq)]
pub enum SnowflakeGeneratorError {
    #[error("Node ID is too large")]
    NodeIdTooLarge,
    #[error("Invalid system clock")]
    InvalidSystemClock,
    #[error("Environment variable error")]
    EnvVarError(#[from] VarError),
    #[error("Parse int error")]
    ParseIntError(#[from] ParseIntError),
}
