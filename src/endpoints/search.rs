use crate::{
    GammaClient, RestError,
    types::types_search::{PublicSearchArgs, PublicSearchResponse},
};

impl GammaClient {
    pub async fn public_search(
        &self,
        args: PublicSearchArgs,
    ) -> Result<PublicSearchResponse, RestError> {
        let path = format!("{}/public-search", self.base_url);
        let response = self
            .http_client
            .get(path)
            .query(&args)
            .send()
            .await
            .map_err(RestError::RequestError)?;

        let status = response.status();
        let raw_text = response.text().await.map_err(RestError::RequestError)?;

        if !status.is_success() {
            return Err(RestError::HttpError {
                status,
                body: raw_text,
            });
        }

        serde_json::from_str(&raw_text).map_err(|e| RestError::ParseError {
            error: e,
            raw_json: raw_text,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{GammaClient, types::types_search::PublicSearchArgs};

    #[tokio::test]
    pub async fn public_search_test() {
        let client = GammaClient::new();

        let args = PublicSearchArgs {
            q: "bitcoin".to_string(),
            cache: None,
            events_status: None,
            limit_per_type: None,
            page: None,
            events_tag: None,
            keep_closed_markets: None,
            sort: None,
            ascending: None,
            search_tags: None,
            search_profiles: None,
            recurrence: None,
            exclude_tag_id: None,
            optimized: None,
        };

        let response = client.public_search(args).await;

        println!("{:?}", response);

        assert!(response.is_ok());
    }
}
