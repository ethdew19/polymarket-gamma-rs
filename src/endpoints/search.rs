use crate::{
    client::GammaClient,
    error::Result,
    types::types_search::{PublicSearchArgs, PublicSearchResponse},
};

impl GammaClient {
    pub async fn public_search(&self, args: &PublicSearchArgs) -> Result<PublicSearchResponse> {
        self.get("public-search", args).await
    }
}

#[cfg(test)]
mod tests {
    use crate::{client::GammaClient, types::types_search::PublicSearchArgs};

    #[tokio::test]
    pub async fn public_search_test() {
        let client = GammaClient::default();

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

        let response = client.public_search(&args).await;

        println!("{:?}", response);

        assert!(response.is_ok());
    }
}
