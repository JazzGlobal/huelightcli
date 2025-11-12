use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SuccessDetail {
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct ErrorDetail {
    #[serde(rename = "type")]
    pub r#type: u32,
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
