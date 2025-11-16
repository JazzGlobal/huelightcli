use crate::error::CoreError;

pub trait HueClient {
    fn post_json(
        &self,
        url: &str,
        body: &str,
    ) -> impl std::future::Future<Output = Result<String, CoreError>> + Send;
    fn get(&self, url: &str)
    -> impl std::future::Future<Output = Result<String, CoreError>> + Send;
    fn put_json(
        &self,
        url: &str,
        body: &str,
    ) -> impl std::future::Future<Output = Result<String, CoreError>> + Send;
}

pub struct ReqwestHueClient {
    pub client: reqwest::Client,
}

impl HueClient for ReqwestHueClient {
    async fn post_json(&self, url: &str, body: &str) -> Result<String, CoreError> {
        // Implementation for sending a POST request with JSON body
        let res = self
            .client
            .post(url)
            .header("Content-Type", "application/json")
            .body(body.to_string())
            .send()
            .await
            .map_err(CoreError::Network)?;

        Ok(res.text().await?)
    }

    async fn get(&self, url: &str) -> Result<String, CoreError> {
        let res = self
            .client
            .get(url)
            .send()
            .await
            .map_err(CoreError::Network)?;
        Ok(res.text().await?)
    }

    async fn put_json(&self, url: &str, body: &str) -> Result<String, CoreError> {
        let res = self
            .client
            .put(url)
            .header("Content-Type", "application/json")
            .body(body.to_string())
            .send()
            .await
            .map_err(CoreError::Network)?;

        Ok(res.text().await?)
    }
}
