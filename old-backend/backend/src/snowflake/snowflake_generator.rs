use crate::snowflake::snowflake_error::SnowflakeGeneratorError;
use crate::time::timestamp::get_epoch_millis;
use std::sync::atomic::{AtomicU64, Ordering};

pub(crate) const SNOWFLAKE_EPOCH: i64 = 1_705_247_483_000;

pub(crate) const NODE_ID_BITS: u8 = 10;
pub(crate) const SEQUENCE_BITS: u8 = 12;

pub(crate) const TIMESTAMP_SHIFT: u8 = NODE_ID_BITS + SEQUENCE_BITS;

pub(crate) const MAX_NODE_ID: u64 = (1 << NODE_ID_BITS) - 1;
pub(crate) const MAX_SEQUENCE: u64 = (1 << SEQUENCE_BITS) - 1;

#[derive(Debug)]
pub(crate) struct SnowflakeGenerator {
    node_id: u64,
    last_timestamp: AtomicU64,
    sequence: AtomicU64,
}

impl SnowflakeGenerator {
    pub(crate) fn new(node_id: u64) -> Self {
        Self {
            node_id,
            last_timestamp: AtomicU64::new(0),
            sequence: AtomicU64::new(0),
        }
    }

    pub(crate) fn next_id(&self) -> Result<i64, SnowflakeGeneratorError> {
        let mut current_timestamp = self.timestamp();
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

    fn timestamp(&self) -> u64 {
        (get_epoch_millis() - (SNOWFLAKE_EPOCH as u128)) as u64
    }

    fn wait_for_next_millis(
        &self,
        mut current_timestamp: u64,
        last_timestamp: u64,
    ) -> Result<u64, SnowflakeGeneratorError> {
        while current_timestamp == last_timestamp {
            current_timestamp = self.timestamp();
        }

        Ok(current_timestamp)
    }
}
