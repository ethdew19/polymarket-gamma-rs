use serde::{Deserialize, Serialize};

pub type GetSportsMetadataResponse = Vec<SportsMetaData>;

#[derive(Debug, Clone, Deserialize)]
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
    pub limit: Option<u64>,
    pub offset: Option<u64>,
    pub order: Option<String>,
    pub ascending: Option<bool>,
    pub league: Option<Vec<String>>,
    pub name: Option<Vec<String>>,
    pub abbreviation: Option<Vec<String>>,
}

pub type ListTeamsResponse = Vec<ListedTeam>;

#[derive(Debug, Clone, Deserialize)]
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

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetValidSportsMarketTypesResponse {
    pub market_types: Vec<String>,
}
