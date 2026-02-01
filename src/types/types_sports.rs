use serde::{Deserialize, Serialize};

pub type GetSportsMetadataResponse = Vec<SportsMetaData>;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SportsMetaData {
    pub id: u32, // id isn't listed in docs...
    pub sport: String,
    pub image: String,
    pub resolution: String,
    pub ordering: String,
    pub tags: String,
    pub series: String,
    pub created_at: String, // also not listed in docs
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct ListTeamsArgs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ascending: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub league: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abbreviation: Option<Vec<String>>,
}

pub type ListTeamsResponse = Vec<ListedTeam>;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListedTeam {
    pub id: u64,
    pub name: Option<String>,
    pub league: Option<String>,
    pub record: Option<String>,
    pub logo: Option<String>,
    pub abbreviation: Option<String>,
    pub alias: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub provider_id: Option<u32>, // Not in the docs but in the actual response
    pub color: Option<String>,    // Also not in the docs
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetValidSportsMarketTypesResponse {
    pub market_types: Vec<String>,
}
