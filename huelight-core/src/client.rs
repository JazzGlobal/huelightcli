use crate::models::{CreateUserEntry, CreateUserResponse, LightResponse, LightState, User};
use anyhow::Context;

pub trait HueClient {
    fn post_json(
        &self,
        url: &str,
        body: &str,
    ) -> impl std::future::Future<Output = anyhow::Result<String>> + Send;
    fn get(&self, url: &str) -> impl std::future::Future<Output = anyhow::Result<String>> + Send;
    fn put_json(
        &self,
        url: &str,
        body: &str,
    ) -> impl std::future::Future<Output = anyhow::Result<String>> + Send;
}

pub struct ReqwestHueClient {
    pub client: reqwest::Client,
}

impl HueClient for ReqwestHueClient {
    async fn post_json(&self, url: &str, body: &str) -> anyhow::Result<String> {
        // Implementation for sending a POST request with JSON body
        let res = self
            .client
            .post(url)
            .header("Content-Type", "application/json")
            .body(body.to_string())
            .send()
            .await?;

        Ok(res.text().await?)
    }

    async fn get(&self, url: &str) -> anyhow::Result<String> {
        let res = self.client.get(url).send().await?;
        Ok(res.text().await?)
    }

    async fn put_json(&self, url: &str, body: &str) -> anyhow::Result<String> {
        let res = self
            .client
            .put(url)
            .header("Content-Type", "application/json")
            .body(body.to_string())
            .send()
            .await?;

        Ok(res.text().await?)
    }
}

pub trait ILogger {
    fn log(&mut self, message: &str);
}

pub struct Logger {
    // Logger implementation goes here
    pub log: Vec<String>,
}

impl ILogger for Logger {
    fn log(&mut self, message: &str) {
        /*
         * Logs a message to the logger's internal log storage.
         * Puts a newline after each message.
         */
        self.log.push(message.to_string() + "\n");
        println!("{}", message);
    }
}

pub async fn async_create_user(
    ip_address: &str,
    device_name: &str,
    client: &impl HueClient,
    logger: &mut impl ILogger,
) -> anyhow::Result<()> {
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

    let parsed: CreateUserResponse =
        serde_json::from_str(&res).context("parsing Hue create-user response")?;

    match parsed.first() {
        Some(CreateUserEntry::Success { success }) => {
            let message = format!("User created successfully! Username: {}", success.username);
            logger.log(&message);
        }
        Some(CreateUserEntry::Error { error }) => {
            let message = format!(
                "Error creating user: {} - {}",
                error.address, error.description
            );
            logger.log(&message);
        }
        None => {
            println!("Unexpected response from Hue Bridge.");
        }
    }

    Ok(())
}

pub async fn async_get_all_lights(
    ip_address: &str,
    username: &str,
    client: &impl HueClient,
    logger: &mut impl ILogger,
) -> anyhow::Result<()> {
    /*
     * Sends a get request to the input IP Address of the Hue Bridge to retrieve all lights connected to the bridge.
     */

    let url = format!("http://{}/api/{}/lights", ip_address, username);
    let res = client.get(&url).await?;
    let parsed: LightResponse =
        serde_json::from_str(&res).context("parsing /lights GET response")?;

    for (id, light) in parsed.0 {
        logger.log(&format!(
            "Light ID: {}, On: {}, Name: {}, Type: {}, Brightness: {}, Hue: {}, Saturation: {}",
            id,
            light.state.on.unwrap_or(false),
            light.name,
            light._type,
            light.state.brightness.unwrap_or(0),
            light.state.hue.unwrap_or(0),
            light.state.saturation.unwrap_or(0)
        ));
    }

    Ok(())
}

pub async fn async_set_light_state(
    ip_address: &str,
    username: &str,
    light_id: u32,
    state: &LightState,
    client: &impl HueClient,
    logger: &mut impl ILogger,
) -> anyhow::Result<()> {
    /*
     * Sends a PUT request to change the state of a specific light.
     */

    let url = format!(
        "http://{}/api/{}/lights/{}/state",
        ip_address, username, light_id
    );
    let json_state = serde_json::to_string(&state).context("serializing light state")?;

    let res = client.put_json(&url, &json_state).await?;

    logger.log(&format!(
        "Response from setting light {} state: {}",
        light_id, res
    ));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{async_create_user, async_get_all_lights};
    use crate::client::{HueClient, Logger};

    #[tokio::test]
    async fn async_create_user_successresponse_logs_username() {
        // Arrange
        struct FakeClient {}

        impl HueClient for FakeClient {
            async fn post_json(&self, _url: &str, _body: &str) -> anyhow::Result<String> {
                let fake_response = r#"[{"success":{"username":"testusername"}}]"#;
                Ok(fake_response.to_string())
            }

            async fn get(&self, _url: &str) -> anyhow::Result<String> {
                Ok("".to_string())
            }

            async fn put_json(&self, _url: &str, _body: &str) -> anyhow::Result<String> {
                Ok("".to_string())
            }
        }
        let mut logger = Logger { log: Vec::new() };
        let fake_client = FakeClient {};

        // Act
        let result = async_create_user("127.0.0.1", "device", &fake_client, &mut logger).await;

        // Assert
        assert!(result.is_ok());
        assert!(
            logger
                .log
                .iter()
                .any(|entry| entry.contains("User created successfully! Username: testusername"))
        );
    }

    #[tokio::test]
    async fn async_create_user_errorresponse_logs_error() {
        // Arrange
        struct FakeClient {}

        impl HueClient for FakeClient {
            async fn post_json(&self, _url: &str, _body: &str) -> anyhow::Result<String> {
                let fake_response = r#"[{"error":{"type":101,"address":"/","description":"link button not pressed"}}]"#;
                Ok(fake_response.to_string())
            }

            async fn get(&self, _url: &str) -> anyhow::Result<String> {
                Ok("".to_string())
            }

            async fn put_json(&self, _url: &str, _body: &str) -> anyhow::Result<String> {
                Ok("".to_string())
            }
        }
        let mut logger = Logger { log: Vec::new() };
        let fake_client = FakeClient {};

        // Act
        let result = async_create_user("127.0.0.1", "device", &fake_client, &mut logger).await;

        // Assert
        assert!(result.is_ok());
        assert!(
            logger
                .log
                .iter()
                .any(|entry| entry.contains("Error creating user: / - link button not pressed"))
        );
    }

    #[tokio::test]
    async fn async_get_all_lights_logs_light_information() {
        // Arrange
        struct FakeClient {}

        impl HueClient for FakeClient {
            async fn post_json(&self, _url: &str, _body: &str) -> anyhow::Result<String> {
                Ok("".to_string())
            }

            // Setup get to return the expected lights JSON from Hue Bridge's /lights endpoint
            async fn get(&self, _url: &str) -> anyhow::Result<String> {
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

            async fn put_json(&self, _url: &str, _body: &str) -> anyhow::Result<String> {
                Ok("".to_string())
            }
        }

        let mut logger = Logger { log: Vec::new() };
        let fake_client = FakeClient {};

        // Act
        // The username doesn't matter because the FakeClient doesn't use it.
        let result = async_get_all_lights("127.0.0.1", "", &fake_client, &mut logger).await;

        // Assert
        assert!(result.is_ok());
        assert!(
            logger
                .log
                .iter()
                .any(|entry| entry.contains("Light ID: 1, On: true, Name: Living Room Light, Type: Extended color light, Brightness: 200, Hue: 50000, Saturation: 150"))
        );
        assert!(
            logger
                .log
                .iter()
                .any(|entry| entry.contains("Light ID: 2, On: false, Name: Bedroom Light, Type: Dimmable light, Brightness: 100, Hue: 30000, Saturation: 100"))
        );
    }
}
