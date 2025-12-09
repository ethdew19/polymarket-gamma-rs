use serde::Serialize;

use crate::types::types_events::{EventTags, Market};

#[derive(Serialize, Default, Debug, Clone)]
pub struct ListMarketsArgs {
    pub limit: Option<u64>,
    pub offset: Option<u64>,
    pub order: Option<String>,
    pub ascending: Option<bool>,
    pub id: Option<Vec<u64>>,
    pub slug: Option<Vec<String>>,
    pub clob_token_ids: Option<Vec<String>>,
    pub condition_ids: Option<Vec<String>>,
    pub market_maker_address: Option<Vec<String>>,
    pub liquiditity_num_min: Option<u64>,
    pub liquiditity_num_max: Option<u64>,
    pub volume_num_min: Option<f64>,
    pub volume_num_max: Option<f64>,
    pub start_date_min: Option<String>,
    pub start_date_max: Option<String>,
    pub end_date_min: Option<String>,
    pub end_date_max: Option<String>,
    pub tag_id: Option<i64>,
    pub related_tags: Option<bool>,
    pub cyom: Option<bool>,
    pub uma_resolution_status: Option<String>,
    pub game_id: Option<String>,
    pub sports_market_types: Option<Vec<String>>,
    pub rewards_min_size: Option<f64>,
    pub question_ids: Option<Vec<String>>,
    pub include_tag: Option<bool>,
    pub closed: Option<bool>,
}

pub type ListMarketsResponse = Vec<Market>;

#[derive(Serialize, Debug, Clone)]
pub struct GetMarketByIdArgs {
    pub id: u64,
    pub include_tag: Option<bool>,
}

pub type GetMarketByIdResponse = Market;
pub type GetMarketTagsByIdResponse = Vec<EventTags>;

#[derive(Debug, Clone, Serialize)]
pub struct GetMarketBySlugArgs {
    pub slug: String,
    pub include_tag: Option<bool>,
}

pub type GetMarketBySlugResponse = Market;
