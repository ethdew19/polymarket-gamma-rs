use serde::{Deserialize, Serialize};

use super::{
    types_events::{Event, ImageOptimized},
    types_tags::Tag,
};

#[derive(Debug, Clone, Serialize)]
pub struct PublicSearchArgs {
    /// Required search query string
    pub q: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub events_status: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_per_type: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub events_tag: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_closed_markets: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ascending: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_tags: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_profiles: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_tag_id: Option<Vec<i32>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub optimized: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]

pub struct PublicSearchResponse {
    pub events: Option<Vec<Event>>,
    pub tags: Option<Vec<Tag>>,
    pub profiles: Option<Vec<Profile>>,
    pub pagination: Page,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub profile: Option<String>,
    pub pseudonym: Option<String>,
    pub display_username_public: Option<bool>,
    pub bio: Option<String>,
    pub is_mod: Option<bool>,
    pub is_creator: Option<bool>,
    pub proxy_wallet: Option<String>,
    pub base_address: Option<String>,
    pub profile_image: Option<String>,
    pub profile_image_optimized: Option<ImageOptimized>,
    pub positions: Option<Vec<Position>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub token_id: Option<String>,
    pub position_size: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Page {
    pub has_more: bool,
    pub total_results: u64,
}
