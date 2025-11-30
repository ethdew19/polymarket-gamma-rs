pub mod endpoints;
pub mod types;
pub struct GammaClient {
    pub base_url: String,
    pub http_client: reqwest::Client,
}

impl Default for GammaClient {
    fn default() -> Self {
        Self::new()
    }
}

impl GammaClient {
    pub fn new() -> Self {
        let http_client = reqwest::Client::new();
        GammaClient {
            base_url: "https://gamma-api.polymarket.com".to_string(),
            http_client,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RestError {
    #[error("HTTP request failed: {0}")]
    RequestError(#[source] reqwest::Error),

    #[error("HTTP {status} error: {body}")]
    HttpError {
        status: reqwest::StatusCode,
        body: String,
    },

    #[error("Failed to parse JSON: {error}\nRaw response: {raw_json}")]
    ParseError {
        #[source]
        error: serde_json::Error,
        raw_json: String,
    },
}

#[cfg(test)]
mod tests {}
