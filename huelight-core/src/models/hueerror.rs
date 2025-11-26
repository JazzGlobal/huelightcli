use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct ErrorDetail {
    #[serde(rename = "type")]
    pub _type: i32,
    pub address: String,
    pub description: String,
}

pub type HueSuccessDetail = HashMap<String, Value>;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum HueResponseEntry {
    Error { error: ErrorDetail },
    Success { success: HueSuccessDetail },
}

pub type HueResponse = Vec<HueResponseEntry>;
