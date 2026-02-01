use crate::{
    error::{RestError, Result},
    types::ToQueryParams,
};
use reqwest::{Method, Request};
use serde::{Serialize, de::DeserializeOwned};

const BASE_URL: &str = "https://gamma-api.polymarket.com";

pub struct GammaClient {
    pub base_url: String,
    pub http_client: reqwest::Client,
}

impl Default for GammaClient {
    fn default() -> Self {
        GammaClient::new(BASE_URL)
    }
}

impl GammaClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        GammaClient {
            http_client: reqwest::Client::new(),
            base_url: base_url.into(),
        }
    }

    pub async fn request<Response: DeserializeOwned>(&self, request: Request) -> Result<Response> {
        let response = self
            .http_client
            .execute(request)
            .await
            .map_err(RestError::RequestError)?;

        let status_code = response.status();

        if !status_code.is_success() {
            let message = response.text().await.unwrap_or_default();

            return Err(RestError::HttpError {
                status: status_code,
                body: message,
            });
        }

        let text = response.text().await.map_err(RestError::RequestError)?;

        let response =
            serde_json::from_str::<Response>(&text).map_err(|e| RestError::ParseError {
                error: e,
                raw_json: text.clone(),
            })?;

        Ok(response)
    }

    pub async fn get<Params: Serialize, Res: DeserializeOwned + Serialize>(
        &self,
        path: &str,
        params: &Params,
    ) -> Result<Res> {
        let parms = params.query_params();
        let path = format!("{}/{}{}", self.base_url, path, parms);
        let request = self
            .http_client
            .request(Method::GET, path)
            .build()
            .map_err(RestError::RequestError)?;
        self.request(request).await
    }
}
