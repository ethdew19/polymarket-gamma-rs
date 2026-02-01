use crate::{
    client::GammaClient,
    error::Result,
    types::types_series::{
        GetSeriesByIdArgs, GetSeriesByIdResponse, ListSeriesArgs, ListSeriesResponse,
    },
};

impl GammaClient {
    pub async fn list_series(&self, args: &ListSeriesArgs) -> Result<ListSeriesResponse> {
        self.get("series", args).await
    }

    pub async fn get_series_by_id(
        &self,
        args: &GetSeriesByIdArgs,
    ) -> Result<GetSeriesByIdResponse> {
        self.get(&format!("series/{}", args.id), args).await
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        client::GammaClient,
        types::types_series::{GetSeriesByIdArgs, ListSeriesArgs},
    };

    #[tokio::test]
    pub async fn list_series_test() {
        let client = GammaClient::default();

        let args = ListSeriesArgs::default();

        let response = client.list_series(&args).await;

        println!("{:?}", response);

        assert!(response.is_ok());
    }

    #[tokio::test]
    pub async fn get_series_by_id_test() {
        let client = GammaClient::default();

        let args = GetSeriesByIdArgs {
            id: 10671,
            include_chat: None,
        };

        let response = client.get_series_by_id(&args).await;

        println!("{:?}", response);

        assert!(response.is_ok());
    }
}
