use crate::{
    GammaClient, RestError,
    types::types_markets::{
        GetMarketByIdArgs, GetMarketByIdResponse, GetMarketBySlugArgs, GetMarketBySlugResponse,
        GetMarketTagsByIdResponse, ListMarketsArgs, ListMarketsResponse,
    },
};

impl GammaClient {
    pub async fn list_markets(
        &self,
        args: ListMarketsArgs,
    ) -> Result<ListMarketsResponse, RestError> {
        let path = format!("{}/markets", self.base_url);

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
    pub async fn get_market_by_id(
        &self,
        args: GetMarketByIdArgs,
    ) -> Result<GetMarketByIdResponse, RestError> {
        let path = format!("{}/markets/{}", self.base_url, args.id);

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
    pub async fn get_market_tags_by_id(
        &self,
        id: u64,
    ) -> Result<GetMarketTagsByIdResponse, RestError> {
        let path = format!("{}/markets/{}/tags", self.base_url, id);

        let response = self
            .http_client
            .get(path)
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
    pub async fn get_market_by_slug(
        &self,
        args: GetMarketBySlugArgs,
    ) -> Result<GetMarketBySlugResponse, RestError> {
        let path = format!("{}/markets/slug/{}", self.base_url, args.slug);

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
    use crate::{
        GammaClient,
        types::types_markets::{GetMarketByIdArgs, GetMarketBySlugArgs, ListMarketsArgs},
    };

    #[tokio::test]
    async fn list_markets_test() {
        let client = GammaClient::new();

        let args = ListMarketsArgs::default();

        let response = client.list_markets(args).await;

        println!("{:?}", response);

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn get_market_by_id_test() {
        let client = GammaClient::new();

        let args = GetMarketByIdArgs {
            id: 46,
            include_tag: None,
        };

        let response = client.get_market_by_id(args).await;

        println!("{:?}", response);

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn get_market_tags_by_id_test() {
        let client = GammaClient::new();

        let id = 46;

        let response = client.get_market_tags_by_id(id).await;

        println!("{:?}", response);

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn get_market_by_slug_test() {
        let client = GammaClient::new();

        let args = GetMarketBySlugArgs {
            slug: "who-will-win-the-2020-league-of-legends-world-championship".to_string(),
            include_tag: None,
        };

        let response = client.get_market_by_slug(args).await;

        println!("{:?}", response);

        assert!(response.is_ok());
    }
}
