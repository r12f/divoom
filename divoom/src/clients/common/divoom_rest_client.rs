use crate::{DivoomAPIError, DivoomAPIResult, DivoomServerErrorInfo};
use log::debug;
use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;

pub struct DivoomRestAPIClient {
    pub server_url_base: String,
    pub http_client: reqwest::Client,
}

impl DivoomRestAPIClient {
    pub fn new(server_url_base: String) -> DivoomRestAPIClient {
        DivoomRestAPIClient {
            server_url_base,
            http_client: reqwest::Client::new(),
        }
    }

    pub async fn send_request<TResp: DeserializeOwned>(
        &self,
        url_path: &str,
    ) -> DivoomAPIResult<TResp> {
        let url = format!("{}{}", self.server_url_base, url_path);
        let request = self.http_client.post(url);
        let response = self.send_request_with_request_builder(request).await?;
        Ok(response)
    }

    pub async fn send_request_with_body<TResp: DeserializeOwned>(
        &self,
        url_path: &str,
        body: String,
    ) -> DivoomAPIResult<TResp> {
        let url = format!("{}{}", self.server_url_base, url_path);
        debug!("Sending request: Url = \"{}\", Body = \"{}\"", url, body);

        let request = self.http_client.post(url).body(body);
        let response = self.send_request_with_request_builder(request).await?;
        Ok(response)
    }

    pub async fn send_raw_request_with_body(
        &self,
        url_path: &str,
        body: String,
    ) -> DivoomAPIResult<String> {
        let url = format!("{}{}", self.server_url_base, url_path);
        debug!("Sending request: Url = \"{}\", Body = \"{}\"", url, body);

        let request = self.http_client.post(url).body(body);
        let response = self.send_raw_request_with_request_builder(request).await?;
        Ok(response)
    }

    async fn send_request_with_request_builder<T: DeserializeOwned>(
        &self,
        request: RequestBuilder,
    ) -> DivoomAPIResult<T> {
        let response_text = self.send_raw_request_with_request_builder(request).await?;
        let parsed_response = serde_json::from_str::<T>(&response_text)?;
        Ok(parsed_response)
    }

    async fn send_raw_request_with_request_builder(
        &self,
        request: RequestBuilder,
    ) -> DivoomAPIResult<String> {
        let response = request.send().await?;
        debug!(
            "Response header received: StatusCode = {}",
            response.status().as_u16()
        );

        if response.status() != reqwest::StatusCode::OK {
            return Err(DivoomAPIError::ServerError(DivoomServerErrorInfo {
                http_status_code: response.status().as_u16(),
                error_code: 0,
                error_message: "".into(),
            }));
        }

        let response_text = response.text().await?;
        debug!("Response received: Body = \"{}\"", response_text);

        Ok(response_text)
    }
}
