use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::snowflake::snowflake_type::Snowflake;

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub(crate) struct Phantom<T> {
    snowflake: Snowflake,
    inner: Option<T>,
}

impl<T> Phantom<T> {
    pub(crate) fn new(snowflake: Snowflake) -> Self {
        Self {
            snowflake,
            inner: None,
        }
    }

    pub(crate) fn from_option(snowflake: Option<i64>) -> Option<Self> {
        snowflake.map(Self::new)
    }

    pub(crate) fn get_snowflake(&self) -> Snowflake {
        self.snowflake.clone()
    }
}

impl<T> From<Snowflake> for Phantom<T> {
    fn from(snowflake: Snowflake) -> Self {
        Self::new(snowflake)
    }
}

impl<T> Serialize for Phantom<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.snowflake.serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for Phantom<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let id = Snowflake::deserialize(deserializer)?;
        Ok(Self::new(id))
    }
}