use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct OpensearchConfig {
    pub is_https: bool,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    #[serde(default = "default_disable_proxy")]
    pub disable_proxy: bool,
}

impl OpensearchConfig {
    pub fn get_url(&self) -> String {
        let protocol = if self.is_https { "https" } else { "http" };

        format!("{}://{}:{}", protocol, self.host, self.port)
    }
}

pub fn default_disable_proxy() -> bool {
    true
}
