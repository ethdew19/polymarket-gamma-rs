use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct GetPublicProfileByWalletAddressRequest {
    pub address: String,
}
#[derive(Deserialize, Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPublicProfileByWalletAddressResponse {
    pub created_at: Option<String>,
    pub proxy_wallet: Option<String>,
    pub profile_image: Option<String>,
    pub display_username_public: Option<bool>,
    pub bio: Option<String>,
    pub pseudonym: Option<String>,
    pub name: Option<String>,
    pub users: Option<Vec<User>>,
    pub x_username: Option<String>,
    pub verified_badge: Option<bool>,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct User {
    pub id: String,
    pub creator: bool,
    #[serde(rename = "mod")]
    pub is_mod: bool,
}
