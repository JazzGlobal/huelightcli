use crate::error::{CoreError, CoreResult};
use async_trait::async_trait;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

pub struct Header {
    pub name: String,
    pub value: String,
}

impl Header {
    pub fn new<N, V>(name: N, value: V) -> Self
    where
        N: Into<String>,
        V: Into<String>,
    {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

#[async_trait]
pub trait HueClient {
    async fn post_json(&self, url: &str, body: &str, headers: &[Header]) -> CoreResult<String>;
    async fn get(&self, url: &str, headers: &[Header]) -> CoreResult<String>;
    async fn put_json(&self, url: &str, body: &str, headers: &[Header]) -> CoreResult<String>;
}

pub struct ReqwestHueClient {
    client: reqwest::Client,
}

impl ReqwestHueClient {
    // Require explicitly injecting a reqwest::Client.
    pub fn new(client: reqwest::Client) -> Self {
        Self { client }
    }

    pub fn header_to_header_map(headers: &[Header]) -> CoreResult<HeaderMap> {
        let mut map = HeaderMap::new();
        for h in headers {
            let name = HeaderName::from_bytes(h.name.as_bytes())
                .map_err(CoreError::InvalidReqwestHeaderName)?;
            let value =
                HeaderValue::from_str(&h.value).map_err(CoreError::InvalidReqwestHeaderValue)?;
            map.append(name, value);
        }

        Ok(map)
    }
}

#[async_trait]
impl HueClient for ReqwestHueClient {
    async fn post_json(&self, url: &str, body: &str, headers: &[Header]) -> CoreResult<String> {
        // Implementation for sending a POST request with JSON body

        let headers = ReqwestHueClient::header_to_header_map(headers)?;
        let res = self
            .client
            .post(url)
            .headers(headers)
            .body(body.to_string())
            .send()
            .await
            .map_err(CoreError::Network)?;

        res.text().await.map_err(CoreError::Network)
    }

    async fn get(&self, url: &str, headers: &[Header]) -> CoreResult<String> {
        let headers = ReqwestHueClient::header_to_header_map(headers)?;
        let res = self
            .client
            .get(url)
            .headers(headers)
            .send()
            .await
            .map_err(CoreError::Network)?;

        res.text().await.map_err(CoreError::Network)
    }

    async fn put_json(&self, url: &str, body: &str, headers: &[Header]) -> CoreResult<String> {
        let headers = ReqwestHueClient::header_to_header_map(headers)?;
        let res = self
            .client
            .put(url)
            .headers(headers)
            .body(body.to_string())
            .send()
            .await
            .map_err(CoreError::Network)?;

        res.text().await.map_err(CoreError::Network)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        client::{self, Header},
        error::CoreError,
    };

    #[test]
    fn header_to_header_map_invalid_header_name_gives_invalid_name_error() {
        // Arrange
        let h_name = "";
        let h_value = "value";

        let header = Header::new(h_name, h_value);

        // Act
        let result = client::ReqwestHueClient::header_to_header_map(&vec![header]);

        // Assert
        assert!(matches!(
            result,
            Err(CoreError::InvalidReqwestHeaderName(_))
        ));
    }

    #[test]
    fn header_to_header_map_invalid_header_value_gives_invalid_value_error() {
        // Arrange
        let h_name = "valid-name";
        let h_value = "Control\x01char";

        let header = Header::new(h_name, h_value);

        // Act
        let result = client::ReqwestHueClient::header_to_header_map(&vec![header]);

        // Assert
        assert!(matches!(
            result,
            Err(CoreError::InvalidReqwestHeaderValue(_))
        ));
    }

    #[test]
    fn header_to_header_map_valid_header_value_and_name_gives_headermap() {
        // Arrange
        let h_name = "Content-Type";
        let h_value = "application/json";

        let header = Header::new(h_name, h_value);

        // Act
        let result = client::ReqwestHueClient::header_to_header_map(&vec![header]);

        // Assert
        let header_map = result.unwrap();
        let val = header_map.get(h_name);
        let hv = val.unwrap().to_str().unwrap();
        assert_eq!(hv, h_value);
    }
}
