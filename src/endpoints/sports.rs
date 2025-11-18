use crate::{
    GammaClient, RestError,
    types::types_sports::{GetSportsMetadataResponse, ListTeamsArgs, ListTeamsResponse},
};

impl GammaClient {
    pub async fn list_teams(&self, args: ListTeamsArgs) -> Result<ListTeamsResponse, RestError> {
        let path = format!("{}/teams", self.base_url);

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

    pub async fn get_sports_metadata(&self) -> Result<GetSportsMetadataResponse, RestError> {
        let path = format!("{}/sports", self.base_url);

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
}

#[cfg(test)]
mod tests {

    use crate::{GammaClient, types::types_sports::ListTeamsArgs};

    #[tokio::test]
    async fn list_teams_test() {
        let client = GammaClient::new();

        let args = ListTeamsArgs::default();
        let response = client.list_teams(args).await;

        println!("{:?}", response);

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn get_sports_metadata_test() {
        let client = GammaClient::new();
        let response = client.get_sports_metadata().await;

        println!("{:?}", response);

        assert!(response.is_ok());
    }
}
