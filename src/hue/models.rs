use serde::Deserialize;
use std::collections::HashMap;

// Create User related models
#[derive(Debug, Deserialize)]
pub struct SuccessDetail {
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct ErrorDetail {
    pub address: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum CreateUserEntry {
    Success { success: SuccessDetail },
    Error { error: ErrorDetail },
}

#[derive(serde::Serialize)]
pub struct User {
    pub devicetype: String,
}

// The whole response is an ARRAY of entries
pub type CreateUserResponse = Vec<CreateUserEntry>;

// Light related models
pub type LightId = u32;

#[derive(Debug, Deserialize)]
pub struct LightResponse(pub HashMap<LightId, Light>);

#[derive(Debug, Deserialize)]
pub struct Light {
    pub state: LightState,
    pub name: String,
    #[serde(rename = "type")]
    pub _type: String,
}

#[derive(Debug, Deserialize)]
pub struct LightState {
    pub on: bool,
    #[serde(rename = "bri")]
    pub brightness: u8,
    pub hue: u16,
    #[serde(rename = "sat")]
    pub saturation: u8,
}
