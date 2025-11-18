use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListTagsArgs {
    /// Range: x >= 0
    pub limit: Option<u64>,
    /// Range: x >= 0
    pub offset: Option<u64>,
    /// Comma-separated list of fields to order by
    pub order: Option<String>,
    pub ascending: Option<bool>,
    pub include_template: Option<bool>,
    pub is_carousel: Option<bool>,
}

pub type ListTagsResponse = Vec<Tag>;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Tag {
    pub id: String,
    pub label: Option<String>,
    pub slug: Option<String>,
    #[serde(rename = "forceShow")]
    pub force_show: Option<bool>,
    #[serde(rename = "publishedAt")]
    pub published_at: Option<String>,
    #[serde(rename = "createdBy")]
    pub created_by: Option<i64>,
    #[serde(rename = "updatedBy")]
    pub updated_by: Option<i64>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,
    #[serde(rename = "forceHide")]
    pub force_hide: Option<bool>,
    #[serde(rename = "isCarousel")]
    pub is_carousel: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTagByIdArgs {
    pub id: u64,
    pub include_template: bool,
}

pub type GetTagByIdResponse = Tag;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTagBySlugArgs {
    pub slug: String,
    pub include_template: bool,
}

pub type GetTagBySlugResponse = Tag;
