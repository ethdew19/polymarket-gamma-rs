use crate::{
    client::GammaClient,
    error::Result,
    types::types_sports::{
        GetSportsMetadataResponse, GetValidSportsMarketTypesResponse, ListTeamsArgs,
        ListTeamsResponse,
    },
};

impl GammaClient {
    pub async fn list_teams(&self, args: &ListTeamsArgs) -> Result<ListTeamsResponse> {
        self.get("teams", args).await
    }

    pub async fn get_sports_metadata(&self) -> Result<GetSportsMetadataResponse> {
        self.get("sports", &()).await
    }

    pub async fn get_valid_sports_market_types(&self) -> Result<GetValidSportsMarketTypesResponse> {
        self.get("sports/market-types", &()).await
    }
}

#[cfg(test)]
mod tests {

    use crate::{client::GammaClient, types::types_sports::ListTeamsArgs};

    #[tokio::test]
    async fn list_teams_test() {
        let client = GammaClient::default();

        let mut args = ListTeamsArgs::default();
        args.league = Some(vec!["atp".to_string(), "lol".to_string()]);
        let response = client.list_teams(&args).await;

        println!("{:?}", response);

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn get_sports_metadata_test() {
        let client = GammaClient::default();
        let response = client.get_sports_metadata().await;

        println!("{:?}", response);

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn get_valid_sports_market_types_test() {
        let client = GammaClient::default();
        let response = client.get_valid_sports_market_types().await;

        println!("{:?}", response);

        assert!(response.is_ok());
    }
}
