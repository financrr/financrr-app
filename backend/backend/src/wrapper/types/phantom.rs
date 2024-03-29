use std::future::Future;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::api::error::api::ApiError;

pub(crate) trait Identifiable {
    fn from_id(id: i32) -> impl Future<Output = Result<Self, ApiError>> + Send
    where
        Self: Sized;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Phantom<T: Identifiable + Send + 'static> {
    id: i32,
    inner: Option<T>,
}

impl<T: Identifiable + Send + 'static> Phantom<T> {
    pub(crate) fn new(id: i32) -> Self {
        Self {
            id,
            inner: None,
        }
    }

    pub(crate) fn from_option(id: Option<i32>) -> Option<Self> {
        id.map(|id| Self::new(id))
    }

    pub(crate) async fn get_inner(&mut self) -> Result<&T, ApiError> {
        if self.inner.is_none() {
            self.set_inner(T::from_id(self.id).await?);
        }
        Ok(self.inner.as_ref().unwrap())
    }

    pub(crate) async fn fetch_inner(&self) -> Result<T, ApiError> {
        T::from_id(self.id).await
    }

    pub(crate) fn set_inner(&mut self, inner: T) {
        self.inner = Some(inner);
    }

    pub(crate) fn get_id(&self) -> i32 {
        self.id
    }
}

impl<T: Identifiable + Send + 'static + Serialize> Serialize for Phantom<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.id.serialize(serializer)
    }
}

impl<'de, T: Identifiable + Send + 'static + Deserialize<'de>> Deserialize<'de> for Phantom<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let id = i32::deserialize(deserializer)?;
        Ok(Self::new(id))
    }
}
