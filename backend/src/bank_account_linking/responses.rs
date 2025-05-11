use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct JwtResponse {
    pub access: String,
    pub access_expires: u32,
    pub refresh: String,
    pub refresh_expires: u32,
}

#[derive(Debug, Deserialize)]
pub struct RefreshResponse {
    pub access: String,
    pub access_expires: u32,
}
