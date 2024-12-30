use std::env::var;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::datetime::error::TimeError;
use crate::datetime::get_epoch_millis;
use crate::snowflake::entity::Snowflake;
use crate::snowflake::error::SnowflakeGeneratorError;

pub const FINANCRR_SNOWFLAKE_EPOCH: u64 = 1_705_247_483_000;

pub const NODE_ID_BITS: u8 = 10;
pub const SEQUENCE_BITS: u8 = 12;

pub const MAX_NODE_ID: u64 = (1 << NODE_ID_BITS) - 1;
pub const MAX_SEQUENCE: u64 = (1 << SEQUENCE_BITS) - 1;

#[derive(Debug)]
pub struct SnowflakeGenerator {
    node_id: u64,
    epoch: u64,
    last_timestamp: AtomicU64,
    sequence: AtomicU64,
}

impl SnowflakeGenerator {
    pub fn new(node_id: u64, epoch: u64) -> Result<Self, SnowflakeGeneratorError> {
        if node_id > MAX_NODE_ID {
            return Err(SnowflakeGeneratorError::NodeIdTooLarge);
        }

        Ok(Self {
            node_id,
            epoch,
            last_timestamp: AtomicU64::new(0),
            sequence: AtomicU64::new(0),
        })
    }

    pub fn new_from_env() -> Result<Self, SnowflakeGeneratorError> {
        let node_id = var("NODE_ID").unwrap_or("1".to_string()).parse()?;

        Self::new(node_id, FINANCRR_SNOWFLAKE_EPOCH)
    }

    pub fn next(&self) -> Result<Snowflake, SnowflakeGeneratorError> {
        Ok(Snowflake::new(self.next_id()?))
    }

    pub fn next_id(&self) -> Result<i64, SnowflakeGeneratorError> {
        let mut current_timestamp = self.timestamp()?;
        let last_timestamp = self.last_timestamp.load(Ordering::SeqCst);

        if current_timestamp < last_timestamp {
            return Err(SnowflakeGeneratorError::InvalidSystemClock);
        }

        let mut sequence = self.sequence.load(Ordering::SeqCst);

        if current_timestamp == last_timestamp {
            sequence = (sequence + 1) & MAX_SEQUENCE;
            if sequence == 0 {
                current_timestamp = self.wait_for_next_millis(current_timestamp, last_timestamp)?;
            }
        } else {
            sequence = 0;
        }

        self.last_timestamp.store(current_timestamp, Ordering::SeqCst);
        self.sequence.store(sequence, Ordering::SeqCst);

        Ok(((current_timestamp << (NODE_ID_BITS + SEQUENCE_BITS)) | (self.node_id << SEQUENCE_BITS) | sequence) as i64)
    }

    fn timestamp(&self) -> Result<u64, TimeError> {
        Ok(get_epoch_millis()? - self.epoch)
    }

    fn wait_for_next_millis(
        &self,
        mut current_timestamp: u64,
        last_timestamp: u64,
    ) -> Result<u64, SnowflakeGeneratorError> {
        while current_timestamp == last_timestamp {
            current_timestamp = self.timestamp()?;
        }

        Ok(current_timestamp)
    }
}

#[cfg(test)]
mod tests {
    use std::env::set_var;

    use super::*;

    #[test]
    fn test_new_snowflake_generator() {
        let generator = SnowflakeGenerator::new(1, 0).expect("Failed to create SnowflakeGenerator");
        assert_eq!(generator.node_id, 1);
        assert_eq!(generator.epoch, 0);
        assert_eq!(generator.last_timestamp.load(Ordering::SeqCst), 0);
        assert_eq!(generator.sequence.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn test_new_snowflake_generator_with_large_node_id() {
        let generator = SnowflakeGenerator::new(MAX_NODE_ID + 1, 0);
        assert!(generator.is_err());
    }

    #[test]
    fn test_next_id() {
        let generator = SnowflakeGenerator::new(1, 0).expect("Failed to create SnowflakeGenerator");
        let id1 = generator.next_id().expect("Failed to generate ID");
        let id2 = generator.next_id().expect("Failed to generate ID");
        assert!(id2 > id1);
    }

    #[test]
    fn test_new_snowflake_generator_from_env() {
        set_var("NODE_ID", "1");
        let generator = SnowflakeGenerator::new_from_env().expect("Failed to create SnowflakeGenerator from env");
        assert_eq!(generator.node_id, 1);
        assert_eq!(generator.epoch, FINANCRR_SNOWFLAKE_EPOCH);
        assert_eq!(generator.last_timestamp.load(Ordering::SeqCst), 0);
        assert_eq!(generator.sequence.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn test_next_id_from_env() {
        set_var("NODE_ID", "1");
        let generator = SnowflakeGenerator::new_from_env().expect("Failed to create SnowflakeGenerator from env");
        let id1 = generator.next_id().expect("Failed to generate ID");
        let id2 = generator.next_id().expect("Failed to generate ID");
        assert!(id2 > id1);
    }
}
