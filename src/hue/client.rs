use crate::hue::models::{CreateUserEntry, CreateUserResponse, User};

pub trait HueClient {
    async fn post_json(&self, url: &str, body: &str) -> anyhow::Result<(String)>;
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

        return Ok(res.text().await?);
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

    let parsed: CreateUserResponse = serde_json::from_str(&res).unwrap();

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

#[cfg(test)]
mod tests {
    use super::async_create_user;
    use crate::hue::client::{HueClient, Logger};

    #[tokio::test]
    async fn async_create_user_successresponse_logs_username() {
        // Arrange
        struct FakeClient {}

        impl HueClient for FakeClient {
            async fn post_json(&self, _url: &str, _body: &str) -> anyhow::Result<String> {
                let fake_response = r#"[{"success":{"username":"testusername"}}]"#;
                Ok(fake_response.to_string())
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
}
