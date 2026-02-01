use serde::{Deserialize, Serialize};

use super::types_search::Profile;

#[derive(Debug, Clone, Serialize, Default)]
pub struct ListCommentsArgs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ascending: Option<bool>,

    /// Event, Series, market
    //todo: make this enum
    pub parent_entity_type: String,

    pub parent_entity_id: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_positions: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub holders_only: Option<bool>,
}

pub type ListCommentsResponse = Vec<ListedComment>;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListedComment {
    pub id: String,
    pub body: Option<String>,
    pub parent_entity_type: Option<String>,
    #[serde(rename = "parentEntityID")]
    pub parent_entity_id: Option<u64>,
    #[serde(rename = "parentCommentID")]
    pub parent_comment_id: Option<String>,
    pub user_address: Option<String>,
    pub reply_address: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub profile: Option<Profile>,
    pub reactions: Option<Vec<Reaction>>,
    pub report_count: Option<u64>,
    pub reaction_count: Option<u64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Reaction {
    pub id: String,
    pub comment_id: Option<u64>,
    pub reaction_type: Option<String>,
    pub icon: Option<String>,
    pub user_address: Option<String>,
    pub created_at: Option<String>,
    pub profile: Profile,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct GetCommentByCommentIdArgs {
    #[serde(skip_serializing)] // used in path
    pub id: u64,
    pub get_positions: Option<bool>,
}

pub type GetCommentsByCommentIdResponse = Vec<ListedComment>;

#[derive(Debug, Clone, Serialize, Default)]
pub struct GetCommentsByUserAddressArgs {
    #[serde(skip_serializing)] //used in path
    pub user_address: String,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
    pub order: Option<String>,
    pub ascending: Option<bool>,
}

pub type GetCommentsByUserAddressResponse = Vec<ListedComment>;
