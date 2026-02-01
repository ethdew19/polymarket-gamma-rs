use crate::{
    client::GammaClient,
    error::Result,
    types::types_profiles::{
        GetPublicProfileByWalletAddressRequest, GetPublicProfileByWalletAddressResponse,
    },
};

impl GammaClient {
    pub async fn get_public_profile_by_wallet_address(
        &self,
        args: &GetPublicProfileByWalletAddressRequest,
    ) -> Result<GetPublicProfileByWalletAddressResponse> {
        self.get("public-profile", args).await
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        client::GammaClient, types::types_profiles::GetPublicProfileByWalletAddressRequest,
    };

    #[tokio::test]
    async fn get_public_profile_by_wallet_address_test() {
        let client = GammaClient::default();

        let args = GetPublicProfileByWalletAddressRequest {
            address: "0x56687bf447db6ffa42ffe2204a05edaa20f55839".to_string(),
        };
        let response = client.get_public_profile_by_wallet_address(&args).await;

        println!("{:?}", response);

        assert!(response.is_ok());
    }
}
