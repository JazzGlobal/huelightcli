use crate::error::{CoreError, CoreResult};
use async_trait::async_trait;

#[async_trait]
pub trait HueClient {
    async fn post_json(&self, url: &str, body: &str) -> CoreResult<String>;
    async fn get(&self, url: &str) -> CoreResult<String>;
    async fn put_json(&self, url: &str, body: &str) -> CoreResult<String>;
}

pub struct ReqwestHueClient {
    pub client: reqwest::Client,
}

#[async_trait]
impl HueClient for ReqwestHueClient {
    async fn post_json(&self, url: &str, body: &str) -> CoreResult<String> {
        // Implementation for sending a POST request with JSON body
        let res = self
            .client
            .post(url)
            .header("Content-Type", "application/json")
            .body(body.to_string())
            .send()
            .await
            .map_err(CoreError::Network)?;

        res.text().await.map_err(CoreError::Network)
    }

    async fn get(&self, url: &str) -> CoreResult<String> {
        let res = self
            .client
            .get(url)
            .send()
            .await
            .map_err(CoreError::Network)?;

        res.text().await.map_err(CoreError::Network)
    }

    async fn put_json(&self, url: &str, body: &str) -> CoreResult<String> {
        let res = self
            .client
            .put(url)
            .header("Content-Type", "application/json")
            .body(body.to_string())
            .send()
            .await
            .map_err(CoreError::Network)?;

        res.text().await.map_err(CoreError::Network)
    }
}
