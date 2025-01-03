use std::num::ParseIntError;

pub mod parse_int_error {
    use serde::de::Error;
    use serde::{Deserializer, Serializer};

    use super::ParseIntError;

    pub fn serialize<S>(value: &ParseIntError, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&value.to_string())
    }

    pub fn deserialize<'de, D>(_deserializer: D) -> Result<ParseIntError, D::Error>
    where
        D: Deserializer<'de>,
    {
        Err(Error::custom("ParseIntError cannot be deserialized"))
    }
}
