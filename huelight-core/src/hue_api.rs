use crate::client::HueClient;
use crate::error::{CoreError, HueBridgeError};
use crate::logger::ILogger;
use crate::models::{CreateUserEntry, CreateUserResponse, LightResponse, LightState, User};

pub async fn async_create_user(
    ip_address: &str,
    device_name: &str,
    client: &impl HueClient,
    logger: &mut impl ILogger,
) -> Result<User, CoreError> {
    /*
     * Sends a post request to the input IP Address of the Hue Bridge to create a new user with the given device name.
     */

    let new_user = User {
        devicetype: device_name.to_string(),
    };

    let json_user = serde_json::to_string(&new_user).unwrap();

    // Use the injected client to send the POST request
    let url = format!("http://{}/api", ip_address);
    let res = client.post_json(&url, &json_user).await?;

    let parsed: CreateUserResponse = serde_json::from_str(&res).map_err(|err| {
        logger.log(&format!(
            "Failed to parse CreateUserResponse JSON: {err}. Raw(truncated): {}",
            &res[..res.len().min(200)]
        ));
        CoreError::Serialization(err)
    })?;

    match parsed.first() {
        Some(CreateUserEntry::Success { success }) => {
            let message = format!("User created successfully! Username: {}", success.username);
            logger.log(&message);
            Ok(User {
                devicetype: success.username.clone(),
            })
        }
        Some(CreateUserEntry::Error { error }) => {
            let message = format!(
                "Error creating user: {} - {} - {}",
                error._type, error.address, error.description
            );
            logger.log(&message);
            match error._type {
                101 => Err(CoreError::Bridge(HueBridgeError::LinkButtonNotPressed)),
                _default => Err(CoreError::Bridge(HueBridgeError::Other {
                    code: error._type.to_string(),
                    message: error.description.clone(),
                })),
            }
        }
        None => {
            let message =
                "User could not be created. The Hue Bridge returned an unrecognized JSON format.";
            logger.log(message);
            Err(CoreError::UnexpectedResponse(message.to_string()))
        }
    }
}

pub async fn async_get_all_lights(
    ip_address: &str,
    username: &str,
    client: &impl HueClient,
    logger: &mut impl ILogger,
) -> Result<LightResponse, CoreError> {
    /*
     * Sends a get request to the input IP Address of the Hue Bridge to retrieve all lights connected to the bridge.
     */

    let url = format!("http://{}/api/{}/lights", ip_address, username);
    let res = client.get(&url).await?;
    let parsed = serde_json::from_str::<LightResponse>(&res).map_err(|err| {
        logger.log(&format!(
            "Failed to parse lights JSON: {err}. Raw (truncated): {}",
            &res[..res.len().min(200)]
        ));
        CoreError::Serialization(err)
    })?;

    Ok(parsed)
}

pub async fn async_set_light_state(
    ip_address: &str,
    username: &str,
    light_id: u32,
    state: &LightState,
    client: &impl HueClient,
    logger: &mut impl ILogger,
) -> Result<String, CoreError> {
    /*
     * Sends a PUT request to change the state of a specific light.
     */

    let url = format!(
        "http://{}/api/{}/lights/{}/state",
        ip_address, username, light_id
    );
    let json_state = serde_json::to_string(&state).map_err(CoreError::Serialization)?;
    let res = client.put_json(&url, &json_state).await?;

    let message = format!("Response from setting light {} state: {}", light_id, res);
    logger.log(&message);
    Ok(message)
}

#[cfg(test)]
mod tests {
    use super::{async_create_user, async_get_all_lights};
    use crate::client::HueClient;
    use crate::error::{CoreError, HueBridgeError};
    use crate::logger::{ILogger, Logger};
    use crate::models::LightState;

    #[tokio::test]
    async fn async_create_user_successresponse_logs_username() {
        // Arrange
        struct FakeClient {}

        impl HueClient for FakeClient {
            async fn post_json(&self, _url: &str, _body: &str) -> Result<String, CoreError> {
                let fake_response = r#"[{"success":{"username":"testusername"}}]"#;
                Ok(fake_response.to_string())
            }

            async fn get(&self, _url: &str) -> Result<String, CoreError> {
                Ok("".to_string())
            }

            async fn put_json(&self, _url: &str, _body: &str) -> Result<String, CoreError> {
                Ok("".to_string())
            }
        }
        let mut logger = Logger::default();
        let fake_client = FakeClient {};

        // Act
        let result = async_create_user("127.0.0.1", "device", &fake_client, &mut logger).await;

        // Assert
        assert!(result.is_ok());
        assert!(
            logger
                .entries()
                .iter()
                .any(|entry| entry.contains("User created successfully! Username: testusername"))
        );
    }

    #[tokio::test]
    async fn async_create_user_errorresponse_logs_error() {
        // Arrange
        struct FakeClient {}

        impl HueClient for FakeClient {
            async fn post_json(&self, _url: &str, _body: &str) -> Result<String, CoreError> {
                let fake_response = r#"[{"error":{"type":101,"address":"/","description":"link button not pressed"}}]"#;
                Ok(fake_response.to_string())
            }

            async fn get(&self, _url: &str) -> Result<String, CoreError> {
                Ok("".to_string())
            }

            async fn put_json(&self, _url: &str, _body: &str) -> Result<String, CoreError> {
                Ok("".to_string())
            }
        }
        let mut logger = Logger::default();
        let fake_client = FakeClient {};

        // Act
        let result = async_create_user("127.0.0.1", "device", &fake_client, &mut logger).await;

        // Assert
        assert!(matches!(
            result,
            Err(CoreError::Bridge(HueBridgeError::LinkButtonNotPressed))
        ))
    }

    #[tokio::test]
    async fn async_get_all_lights_logs_light_information() {
        // Arrange
        struct FakeClient {}

        impl HueClient for FakeClient {
            async fn post_json(&self, _url: &str, _body: &str) -> Result<String, CoreError> {
                Ok("".to_string())
            }

            // Setup get to return the expected lights JSON from Hue Bridge's /lights endpoint
            async fn get(&self, _url: &str) -> Result<String, CoreError> {
                let fake_response = r#"{
                    "1": {
                        "state": {
                            "on": true,
                            "bri": 200,
                            "hue": 50000,
                            "sat": 150
                        },
                        "name": "Living Room Light",
                        "type": "Extended color light"
                    },
                    "2": {
                        "state": {
                            "on": false,
                            "bri": 100,
                            "hue": 30000,
                            "sat": 100
                        },
                        "name": "Bedroom Light",
                        "type": "Dimmable light"
                    }
                }"#;
                Ok(fake_response.to_string())
            }

            async fn put_json(&self, _url: &str, _body: &str) -> Result<String, CoreError> {
                Ok("".to_string())
            }
        }

        let mut logger = Logger::default();
        let fake_client = FakeClient {};

        // Act
        // The username doesn't matter because the FakeClient doesn't use it.
        let result = async_get_all_lights("127.0.0.1", "", &fake_client, &mut logger).await;

        // Assert

        let parsed_result = result.unwrap();
        let expected_light1 = crate::models::Light {
            name: "Living Room Light".to_string(),
            _type: "Extended color light".to_string(),
            state: LightState {
                on: Some(true),
                brightness: Some(200),
                hue: Some(50000),
                saturation: Some(150),
            },
        };
        let expected_light2 = crate::models::Light {
            name: "Bedroom Light".to_string(),
            _type: "Dimmable light".to_string(),
            state: LightState {
                on: Some(false),
                brightness: Some(100),
                hue: Some(30000),
                saturation: Some(100),
            },
        };

        let light1 = parsed_result.0.get(&1).unwrap();
        let light2 = parsed_result.0.get(&2).unwrap();

        assert_eq!(light1, &expected_light1);
        assert_eq!(light2, &expected_light2);
    }
}
