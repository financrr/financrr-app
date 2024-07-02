use std::env::VarError;

pub mod var_error {
    use serde::de::Error;
    use serde::{Deserializer, Serializer};

    use super::VarError;

    pub fn serialize<S>(value: &VarError, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&value.to_string())
    }

    pub fn deserialize<'de, D>(_deserializer: D) -> Result<VarError, D::Error>
    where
        D: Deserializer<'de>,
    {
        Err(Error::custom("VarError cannot be deserialized"))
    }
}
