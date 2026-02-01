use crate::{
    client::GammaClient,
    error::Result,
    types::types_markets::{
        GetMarketByIdArgs, GetMarketByIdResponse, GetMarketBySlugArgs, GetMarketBySlugResponse,
        GetMarketTagsByIdResponse, ListMarketsArgs, ListMarketsResponse,
    },
};

impl GammaClient {
    pub async fn list_markets(&self, args: &ListMarketsArgs) -> Result<ListMarketsResponse> {
        self.get("markets", args).await
    }

    pub async fn get_market_by_id(
        &self,
        args: &GetMarketByIdArgs,
    ) -> Result<GetMarketByIdResponse> {
        self.get(&format!("markets/{}", args.id), args).await
    }

    pub async fn get_market_tags_by_id(&self, id: u64) -> Result<GetMarketTagsByIdResponse> {
        self.get(&format!("markets/{}/tags", id), &()).await
    }

    pub async fn get_market_by_slug(
        &self,
        args: &GetMarketBySlugArgs,
    ) -> Result<GetMarketBySlugResponse> {
        self.get(&format!("markets/slug/{}", args.slug), args).await
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        client::GammaClient,
        types::types_markets::{GetMarketByIdArgs, GetMarketBySlugArgs, ListMarketsArgs},
    };

    #[tokio::test]
    async fn list_markets_test() {
        let client = GammaClient::default();

        let args = ListMarketsArgs::default();

        let response = client.list_markets(&args).await;

        println!("{:?}", response);

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn get_market_by_id_test() {
        let client = GammaClient::default();

        let args = GetMarketByIdArgs {
            id: 46,
            include_tag: None,
        };

        let response = client.get_market_by_id(&args).await;

        println!("{:?}", response);

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn get_market_tags_by_id_test() {
        let client = GammaClient::default();

        let id = 46;

        let response = client.get_market_tags_by_id(id).await;

        println!("{:?}", response);

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn get_market_by_slug_test() {
        let client = GammaClient::default();

        let args = GetMarketBySlugArgs {
            slug: "who-will-win-the-2020-league-of-legends-world-championship".to_string(),
            include_tag: None,
        };

        let response = client.get_market_by_slug(&args).await;

        println!("{:?}", response);

        assert!(response.is_ok());
    }
}
