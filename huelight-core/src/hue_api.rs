use std::sync::Arc;

use async_trait::async_trait;

use crate::client::{Header, HueClient};
use crate::error::{CoreError, CoreResult, HueBridgeError};
use crate::logger::ILogger;
use crate::models::createuser::{CreateUserEntry, CreateUserResponse, User};
use crate::models::hueerror::HueResponse;
use crate::models::light::{LightResponse, LightState};

#[async_trait]
pub trait HueApi {
    async fn async_get_all_lights(
        &self,
        ip_address: &str,
        username: &str,
    ) -> CoreResult<LightResponse>;
    async fn async_set_light_state(
        &self,
        ip_address: &str,
        username: &str,
        light_id: u32,
        state: &LightState,
    ) -> CoreResult<HueResponse>;
}

pub struct HueApiV1 {
    client: Arc<dyn HueClient + Send + Sync>,
    logger: Arc<dyn ILogger + Send + Sync>,
}

impl HueApiV1 {
    pub fn new(
        client: Arc<dyn HueClient + Send + Sync>,
        logger: Arc<dyn ILogger + Send + Sync>,
    ) -> Self {
        Self { client, logger }
    }
}

#[async_trait]
impl HueApi for HueApiV1 {
    async fn async_get_all_lights(
        &self,
        ip_address: &str,
        username: &str,
    ) -> CoreResult<LightResponse> {
        /*
         * Sends a get request to the input IP Address of the Hue Bridge to retrieve all lights connected to the bridge.
         */

        let url = format!("http://{}/api/{}/lights", ip_address, username);
        let res = self.client.get(&url, Vec::new()).await?;
        let parsed = serde_json::from_str::<LightResponse>(&res).map_err(|err| {
            self.logger.log(&format!(
                "Failed to parse lights JSON: {err}. Raw (truncated): {}",
                &res[..res.len().min(200)]
            ));
            CoreError::Serialization(err)
        })?;

        Ok(parsed)
    }

    async fn async_set_light_state(
        &self,
        ip_address: &str,
        username: &str,
        light_id: u32,
        state: &LightState,
    ) -> CoreResult<HueResponse> {
        /*
         * Sends a PUT request to change the state of a specific light.
         */

        let url = format!(
            "http://{}/api/{}/lights/{}/state",
            ip_address, username, light_id
        );
        let json_state = serde_json::to_string(&state).map_err(CoreError::Serialization)?;
        let headers = vec![Header::new("Content-Type", "application/json")];
        let res = self.client.put_json(&url, &json_state, headers).await?;
        let hue_response_list =
            serde_json::from_str::<HueResponse>(&res).map_err(CoreError::Serialization)?;
        Ok(hue_response_list)
    }
}

pub async fn async_create_user(
    ip_address: &str,
    device_name: &str,
    client: &impl HueClient,
    logger: &mut impl ILogger,
) -> CoreResult<User> {
    /*
     * Sends a post request to the input IP Address of the Hue Bridge to create a new user with the given device name.
     */

    let new_user = User::with_devicetype(device_name);

    let json_user = serde_json::to_string(&new_user).unwrap();

    // Use the injected client to send the POST request
    let url = format!("http://{}/api", ip_address);
    let headers = vec![Header::new("Content-Type", "application/json")];
    let res = client.post_json(&url, &json_user, headers).await?;

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

            Ok(User::with_username(success.username.clone()))
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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::{HueApi, HueApiV1, async_create_user};
    use crate::client::{Header, HueClient};
    use crate::error::{CoreError, CoreResult, HueBridgeError};
    use crate::logger::{ILogger, Logger};
    use crate::models::hueerror::HueResponseEntry;
    use crate::models::light::{Light, LightState};
    use async_trait::async_trait;

    /// Closure used to mock out behavior in the MockHueClient for HueClient.get
    pub type GetFn = Box<dyn Fn(&str) -> CoreResult<String> + Send + Sync>;
    /// Closure used to mock out behavior in the MockHueClient for HueClient.post_json
    pub type PostJsonFn = Box<dyn Fn(&str, &str) -> CoreResult<String> + Send + Sync>;
    /// Closure used to mock out behavior in the MockHueClient for HueClient.put_json
    pub type PutJsonFn = Box<dyn Fn(&str, &str) -> CoreResult<String> + Send + Sync>;

    struct MockHueClient {
        pub post_json_fn: PostJsonFn,
        pub get_fn: GetFn,
        pub put_json_fn: PutJsonFn,
    }

    impl MockHueClient {
        /// Initializes a new MockHueClient with 'Ok' returns for get, put_json, and post_json.
        pub fn new() -> Self {
            Self {
                post_json_fn: Box::new(|_, _| Ok("[]".to_string())),
                get_fn: Box::new(|_| Ok("[]".to_string())),
                put_json_fn: Box::new(|_, _| Ok("[]".to_string())),
            }
        }

        /// Provides a means to implement mocked behavior to MockHueClient.post_json
        pub fn with_post_json<F>(mut self, f: F) -> Self
        where
            F: Fn(&str, &str) -> CoreResult<String> + Send + Sync + 'static,
        {
            self.post_json_fn = Box::new(f);
            self
        }

        /// Provides a means to implement mocked behavior to MockHueClient.get
        pub fn with_get<F>(mut self, f: F) -> Self
        where
            F: Fn(&str) -> CoreResult<String> + Send + Sync + 'static,
        {
            self.get_fn = Box::new(f);
            self
        }

        /// Provides a means to implement mocked behavior to MockHueClient.put_json
        pub fn with_put_json<F>(mut self, f: F) -> Self
        where
            F: Fn(&str, &str) -> CoreResult<String> + Send + Sync + 'static,
        {
            self.put_json_fn = Box::new(f);
            self
        }
    }

    #[async_trait]
    impl HueClient for MockHueClient {
        async fn post_json(
            &self,
            url: &str,
            body: &str,
            _headers: Vec<Header>,
        ) -> CoreResult<String> {
            (self.post_json_fn)(url, body)
        }

        async fn get(&self, url: &str, _headers: Vec<Header>) -> CoreResult<String> {
            (self.get_fn)(url)
        }

        async fn put_json(
            &self,
            url: &str,
            body: &str,
            _headers: Vec<Header>,
        ) -> CoreResult<String> {
            (self.put_json_fn)(url, body)
        }
    }

    #[tokio::test]
    async fn async_create_user_successresponse_logs_username() {
        // Arrange
        let mock_hue_client = MockHueClient::new().with_post_json(|_url, _body| {
            let fake_response = r#"[{"success":{"username":"testusername"}}]"#;
            Ok(fake_response.to_string())
        });
        let mut logger = Logger::default();
        // Act
        let result = async_create_user("127.0.0.1", "device", &mock_hue_client, &mut logger).await;

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
        let mock_hue_client = MockHueClient::new().with_post_json(|_url, _body| {
            let fake_response =
                r#"[{"error":{"type":101,"address":"/","description":"link button not pressed"}}]"#;
            Ok(fake_response.to_string())
        });

        let mut logger = Logger::default();

        // Act
        let result = async_create_user("127.0.0.1", "device", &mock_hue_client, &mut logger).await;

        // Assert
        assert!(matches!(
            result,
            Err(CoreError::Bridge(HueBridgeError::LinkButtonNotPressed))
        ))
    }

    #[tokio::test]
    async fn async_get_all_lights_logs_light_information() {
        // Arrange
        let mock_hue_client = Arc::new(MockHueClient::new().with_get(|_url| {
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
        }));

        let logger: Arc<Logger> = Arc::new(Logger::default());

        let api = HueApiV1::new(mock_hue_client, logger);

        // Act
        let result = api.async_get_all_lights("123.12.123", "").await;

        // Assert
        let parsed_result = result.unwrap();
        let expected_light1 = Light {
            name: "Living Room Light".to_string(),
            _type: "Extended color light".to_string(),
            state: LightState::default()
                .with_on(true)
                .with_brightness(200)
                .with_hue(50000)
                .with_saturation(150),
        };

        let expected_light2 = Light {
            name: "Bedroom Light".to_string(),
            _type: "Dimmable light".to_string(),
            state: LightState::default()
                .with_on(false)
                .with_brightness(100)
                .with_hue(30000)
                .with_saturation(100),
        };

        let light1 = parsed_result.0.get(&1).unwrap();
        let light2 = parsed_result.0.get(&2).unwrap();

        assert_eq!(light1, &expected_light1);
        assert_eq!(light2, &expected_light2);
    }

    #[tokio::test]
    async fn async_set_light_state_invalid_response_returns_serialization_error() {
        // Arrange
        let mock_hue_client = Arc::new(
            MockHueClient::new()
                .with_put_json(|_url, _body| Ok("this cannot be serialized".to_string())),
        );
        let logger: Arc<Logger> = Arc::new(Logger::default());

        let state = LightState::default();

        let api = HueApiV1::new(mock_hue_client, logger);

        // Act
        let result = api
            .async_set_light_state("ipaddress", "username", 1, &state)
            .await;

        // Assert
        assert!(matches!(result, Err(CoreError::Serialization(_))));
    }

    #[tokio::test]
    async fn async_set_light_state_valid_response_returns_model() {
        // Arrange
        let mock_hue_client = Arc::new(MockHueClient::new().with_put_json(|_url, _body| {
            let serialized_response = r#"[ { "error": { "type": 7, "address": "/lights/2/state/bri", "description": "invalid value, null,, for parameter, bri" } }, { "success": { "/lights/2/state/on": false } }]"#;            
            Ok(serialized_response.to_string())
        }));
        let logger: Arc<Logger> = Arc::new(Logger::default());
        let state = LightState::default();

        let api = HueApiV1::new(mock_hue_client, logger);

        // Act
        let result = api
            .async_set_light_state("ipaddress", "username", 1, &state)
            .await
            .unwrap();

        // Assert
        assert_eq!(2, result.len());

        let has_success = result
            .iter()
            .any(|e| matches!(e, HueResponseEntry::Success { .. }));
        let has_error = result
            .iter()
            .any(|e| matches!(e, HueResponseEntry::Error { .. }));

        assert!(has_success);
        assert!(has_error);
    }
}
