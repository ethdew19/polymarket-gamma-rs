use serde::{Deserialize, Serialize};

use super::types_events::Series;

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
    pub id: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_chat: Option<bool>,
}

pub type GetSeriesByIdResponse = Series;
