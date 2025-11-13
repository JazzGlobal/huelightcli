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