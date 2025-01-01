use crate::services::instance_handler::{InstanceHandler, InstanceHandlerInner};
use crate::services::Service;
use base64::prelude::BASE64_URL_SAFE;
use base64::Engine;
use loco_rs::app::AppContext;
use rand::rngs::OsRng;
use rand::RngCore;
use sha2::{Digest, Sha256};
use std::sync::{Arc, OnceLock};

pub const DEFAULT_TOKEN_LENGTH: usize = 64;

pub type SecretGenerator = Arc<SecretGeneratorInner>;

pub struct SecretGeneratorInner {
    instance_handler: InstanceHandler,
}

impl Service for SecretGeneratorInner {
    async fn new(ctx: &AppContext) -> loco_rs::Result<Self> {
        Ok(Self {
            instance_handler: InstanceHandlerInner::get_arc(ctx).await?,
        })
    }

    fn get_static_once() -> &'static OnceLock<Arc<Self>> {
        static INSTANCE: OnceLock<Arc<SecretGeneratorInner>> = OnceLock::new();

        &INSTANCE
    }
}

impl SecretGeneratorInner {
    /// Generates a random token with a default length of 64 characters
    pub fn generate_token(&self) -> String {
        self.generate_token_with_length(DEFAULT_TOKEN_LENGTH)
    }

    pub fn generate_token_with_length(&self, length: usize) -> String {
        let mut random_bytes = [0u8; 32];
        OsRng.fill_bytes(&mut random_bytes);

        let mut hasher = Sha256::new();
        hasher.update(self.instance_handler.get_instance_id().to_string().as_bytes());
        hasher.update(random_bytes);
        let hash = hasher.finalize();

        BASE64_URL_SAFE.encode(hash).chars().take(length).collect()
    }
}
