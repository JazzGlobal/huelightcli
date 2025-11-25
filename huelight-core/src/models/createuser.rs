use serde::{Deserialize, Serialize};

// Create User related models
#[derive(Debug, Deserialize)]
pub struct SuccessDetail {
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct ErrorDetail {
    #[serde(rename = "type")]
    pub _type: i32,
    pub address: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum CreateUserEntry {
    Success { success: SuccessDetail },
    Error { error: ErrorDetail },
}

/// Represents a user in the Hue Bridge API.
///
/// This struct is used for both creating users and representing created users:
/// - For user creation requests: set `devicetype` and leave `username` as `None`
/// - For user creation responses: set `username` and leave `devicetype` as `None`
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    devicetype: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<String>,
}

impl User {
    pub fn devicetype(&self) -> Option<&str> {
        self.devicetype.as_deref()
    }

    pub fn username(&self) -> Option<&str> {
        self.username.as_deref()
    }

    pub fn with_devicetype(devicetype: impl Into<String>) -> Self {
        Self {
            devicetype: Some(devicetype.into()),
            username: None,
        }
    }

    pub fn with_username(username: impl Into<String>) -> Self {
        Self {
            username: Some(username.into()),
            devicetype: None,
        }
    }
}

// The whole response is an ARRAY of entries
pub type CreateUserResponse = Vec<CreateUserEntry>;

#[cfg(test)]
mod tests {
    use crate::models::createuser::User;

    #[test]
    pub fn user_serialization_omits_username_when_none() {
        let user = User::with_devicetype("device");
        let serialized = serde_json::to_string(&user).unwrap();
        assert_eq!("{\"devicetype\":\"device\"}".to_string(), serialized);
    }

    #[test]
    pub fn user_serialization_omits_devicetype_when_none() {
        let user = User::with_username("myusername");
        let serialized = serde_json::to_string(&user).unwrap();
        assert_eq!("{\"username\":\"myusername\"}".to_string(), serialized);
    }
}
