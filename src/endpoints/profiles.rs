use crate::{
    GammaClient, RestError, types::types_profiles::GetPublicProfileByWalletAddressResponse,
};

impl GammaClient {
    pub async fn get_public_profile_by_wallet_address(
        &self,
        address: &str,
    ) -> Result<GetPublicProfileByWalletAddressResponse, RestError> {
        let path = format!("{}/public-profile", self.base_url);
        let response = self
            .http_client
            .get(path)
            .query(&[("address", address)])
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
    use crate::GammaClient;

    #[tokio::test]
    async fn get_public_profile_by_wallet_address_test() {
        let client = GammaClient::new();

        let response = client
            .get_public_profile_by_wallet_address("0x56687bf447db6ffa42ffe2204a05edaa20f55839")
            .await;

        println!("{:?}", response);

        assert!(response.is_ok());
    }
}
