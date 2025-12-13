use crate::{
    GammaClient, RestError,
    types::types_series::{
        GetSeriesByIdArgs, GetSeriesByIdResponse, ListSeriesArgs, ListSeriesResponse,
    },
};

impl GammaClient {
    pub async fn list_series(&self, args: ListSeriesArgs) -> Result<ListSeriesResponse, RestError> {
        let path = format!("{}/series", self.base_url);

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
    pub async fn get_series_by_id(
        &self,
        args: GetSeriesByIdArgs,
    ) -> Result<GetSeriesByIdResponse, RestError> {
        let path = format!("{}/series/{}", self.base_url, args.id);

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
        types::types_series::{GetSeriesByIdArgs, ListSeriesArgs},
    };

    #[tokio::test]
    pub async fn list_series_test() {
        let client = GammaClient::new();

        let args = ListSeriesArgs::default();

        let response = client.list_series(args).await;

        println!("{:?}", response);

        assert!(response.is_ok());
    }

    #[tokio::test]
    pub async fn get_series_by_id_test() {
        let client = GammaClient::new();

        let args = GetSeriesByIdArgs {
            id: 10671,
            include_chat: None,
        };

        let response = client.get_series_by_id(args).await;

        println!("{:?}", response);

        assert!(response.is_ok());
    }
}
