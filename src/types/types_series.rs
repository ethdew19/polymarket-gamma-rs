use serde::{Deserialize, Serialize};

use super::{
    types_events::{Category, Chat, Collection, Event},
    types_tags::Tag,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Series {
    pub id: String,
    pub ticker: Option<String>,
    pub slug: Option<String>,
    pub title: Option<String>,
    pub subtitle: Option<String>,

    pub series_type: Option<String>,

    pub recurrence: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
    pub icon: Option<String>,
    pub layout: Option<String>,
    pub active: Option<bool>,
    pub closed: Option<bool>,
    pub archived: Option<bool>,
    pub new: Option<bool>,
    pub featured: Option<bool>,
    pub restricted: Option<bool>,

    pub is_template: Option<bool>,

    pub template_variables: Option<bool>,

    pub published_at: Option<String>,

    pub created_by: Option<String>,

    pub updated_by: Option<String>,

    pub created_at: Option<String>,

    pub updated_at: Option<String>,

    pub comments_enabled: Option<bool>,

    pub competitive: Option<String>,
    pub volume24hr: Option<f64>,
    pub volume: Option<f64>,
    pub liquidity: Option<f64>,

    pub start_date: Option<String>,

    pub pyth_token_id: Option<String>,

    pub cg_asset_name: Option<String>,

    pub score: Option<i32>,
    pub events: Option<Vec<Event>>,
    pub collections: Option<Vec<Collection>>,
    pub categories: Option<Vec<Category>>,
    pub tags: Option<Vec<Tag>>,

    pub comment_count: Option<i32>,

    pub chats: Option<Vec<Chat>>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct ListSeriesArgs {
    /// Limit for pagination (must be >= 0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Offset for pagination (must be >= 0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,

    /// Comma-separated list of fields to order by
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,

    /// Sort in ascending order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ascending: Option<bool>,

    /// Filter by slug(s)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<Vec<String>>,

    /// Filter by category IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories_ids: Option<Vec<i32>>,

    /// Filter by category labels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories_labels: Option<Vec<String>>,

    /// Filter by closed status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed: Option<bool>,

    /// Include chat information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_chat: Option<bool>,

    /// Filter by recurrence
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<String>,
}

pub type ListSeriesResponse = Vec<Series>;

#[derive(Debug, Clone, Serialize)]
pub struct GetSeriesByIdArgs {
    #[serde(skip_serializing)] // used in path
    pub id: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_chat: Option<bool>,
}

pub type GetSeriesByIdResponse = Series;
