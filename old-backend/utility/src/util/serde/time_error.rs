pub mod component_range {
    use serde::de::Error;
    use serde::{Deserializer, Serializer};
    use time::error::ComponentRange;

    pub fn serialize<S>(value: &ComponentRange, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&value.to_string())
    }

    pub fn deserialize<'de, D>(_deserializer: D) -> Result<ComponentRange, D::Error>
    where
        D: Deserializer<'de>,
    {
        Err(Error::custom("ComponentRange cannot be deserialized"))
    }
}

pub mod system_time_error {
    use std::time::SystemTimeError;

    use serde::de::Error;
    use serde::{Deserializer, Serializer};

    pub fn serialize<S>(value: &SystemTimeError, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&value.to_string())
    }

    pub fn deserialize<'de, D>(_deserializer: D) -> Result<SystemTimeError, D::Error>
    where
        D: Deserializer<'de>,
    {
        Err(Error::custom("SystemTimeError cannot be deserialized"))
    }
}

pub mod local_result {
    use chrono::{DateTime, FixedOffset, LocalResult};
    use serde::de::Error;
    use serde::ser::SerializeSeq;
    use serde::{Deserializer, Serializer};

    pub fn serialize<S>(value: &LocalResult<DateTime<FixedOffset>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            LocalResult::Single(datetime) => serializer.serialize_some(datetime),
            LocalResult::None => serializer.serialize_none(),
            LocalResult::Ambiguous(ear, lat) => {
                let mut state = serializer.serialize_seq(None)?;
                state.serialize_element(ear)?;
                state.serialize_element(lat)?;
                state.end()
            }
        }
    }

    pub fn deserialize<'de, D>(_deserializer: D) -> Result<LocalResult<DateTime<FixedOffset>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Err(Error::custom("LocalResult cannot be deserialized"))
    }
}
