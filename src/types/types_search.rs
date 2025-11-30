use serde::{Deserialize, Serialize};

use super::types_event::{Event, Tag};

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
pub struct Profile {}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Page {
    pub has_more: bool,
    pub total_results: u64,
}
