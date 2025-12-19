use serde::{Deserialize, Serialize};

pub type GetSportsMetadataResponse = Vec<SportsMetaData>;

#[derive(Debug, Clone, Deserialize)]
pub struct SportsMetaData {
    pub sport: String,
    pub image: String,
    pub resolution: String,
    pub ordering: String,
    pub tags: String,
    pub series: String,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct ListTeamsArgs {
    limit: Option<u64>,
    offset: Option<u64>,
    order: Option<String>,
    ascending: Option<bool>,
    league: Option<Vec<String>>,
    name: Option<Vec<String>>,
    abbreviation: Option<Vec<String>>,
}

pub type ListTeamsResponse = Vec<ListedTeam>;

#[derive(Debug, Clone, Deserialize)]
pub struct ListedTeam {
    pub id: u64,
    pub name: Option<String>,
    pub league: Option<String>,
    pub record: Option<String>,
    pub logo: Option<String>,
    pub abbreviation: Option<String>,
    pub alias: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetValidSportsMarketTypesResponse {
    pub market_types: Vec<String>,
}
