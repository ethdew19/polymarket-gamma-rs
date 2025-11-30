use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Default)]
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

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub id: String,
    pub label: Option<String>,
    pub slug: Option<String>,
    pub force_show: Option<bool>,
    pub published_at: Option<String>,
    pub created_by: Option<i64>,
    pub updated_by: Option<i64>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub force_hide: Option<bool>,
    pub is_carousel: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetTagByIdArgs {
    pub id: u64,
    pub include_template: bool,
}

pub type GetTagByIdResponse = Tag;

#[derive(Debug, Clone, Serialize)]
pub struct GetTagBySlugArgs {
    #[serde(skip_serializing)]
    pub slug: String,
    pub include_template: bool,
}

pub type GetTagBySlugResponse = Tag;

#[derive(Debug, Clone, Serialize)]
pub struct GetRelatedTagByIdArgs {
    #[serde(skip_serializing)]
    pub id: u64,
    pub omit_empty: Option<bool>,
    pub status: Option<String>,
}

pub type GetRelatedTagByIdResponse = Vec<TagRelationship>;

#[derive(Debug, Clone, Deserialize)]
pub struct TagRelationship {
    pub id: String,
    #[serde(rename = "tagID")]
    pub tag_id: Option<u64>,
    #[serde(rename = "relatedTagID")]
    pub related_tag_id: Option<u64>,
    pub rank: Option<u64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetRelatedTagBySlugArgs {
    pub slug: String,
    pub omit_empty: Option<u64>,
    pub status: Option<String>,
}

pub type GetRelatedTagBySlugResponse = Vec<TagRelationship>;

#[derive(Debug, Clone, Serialize)]
pub struct GetTagsRelatedToIdArgs {
    pub id: u64,
    pub omit_empty: Option<bool>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelatedTag {
    pub id: String,
    pub label: Option<String>,
    pub slug: Option<String>,
    pub force_show: Option<bool>,
    pub published_at: Option<String>,
    pub created_by: Option<u64>,
    pub updated_by: Option<u64>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub force_hide: Option<String>,
    pub is_carousel: Option<bool>,
}
pub type GetTagsRelatedToIdResponse = Vec<RelatedTag>;

#[derive(Debug, Clone, Serialize)]
pub struct GetTagsRelatedToSlugArgs {
    pub slug: String,
    pub omit_empty: Option<bool>,
    pub status: Option<String>,
}

pub type GetTagsRelatedToSlugResponse = Vec<RelatedTag>;
