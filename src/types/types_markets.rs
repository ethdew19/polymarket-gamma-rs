use crate::types::types_events::{Category, Event, EventTags};
use crate::types::types_tags::Tag;
use serde::{Deserialize, Serialize};

use super::types_events::ImageOptimized;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Market {
    pub id: String,
    pub question: Option<String>,

    pub condition_id: String,

    pub slug: Option<String>,

    pub twitter_card_image: Option<String>,

    pub resolution_source: Option<String>,

    pub end_date: Option<String>,

    pub category: Option<String>,

    pub amm_type: Option<String>,

    pub liquidity: Option<String>,

    pub sponsor_name: Option<String>,

    pub sponsor_image: Option<String>,

    pub start_date: Option<String>,

    pub x_axis_value: Option<String>,

    pub y_axis_value: Option<String>,

    pub denomination_token: Option<String>,

    pub fee: Option<String>,
    pub image: Option<String>,
    pub icon: Option<String>,

    pub lower_bound: Option<String>,

    pub upper_bound: Option<String>,

    pub description: Option<String>,
    pub outcomes: Option<String>,

    pub outcome_prices: Option<String>,

    pub volume: Option<String>,
    pub active: Option<bool>,

    pub market_type: Option<String>,

    pub format_type: Option<String>,

    pub lower_bound_date: Option<String>,

    pub upper_bound_date: Option<String>,

    pub closed: Option<bool>,

    pub market_maker_address: String,

    pub created_by: Option<i32>,

    pub updated_by: Option<i32>,

    pub created_at: Option<String>,

    pub updated_at: Option<String>,

    pub closed_time: Option<String>,

    pub wide_format: Option<bool>,

    pub new: Option<bool>,

    pub mailchimp_tag: Option<String>,

    pub featured: Option<bool>,
    pub archived: Option<bool>,

    pub resolved_by: Option<String>,

    pub restricted: Option<bool>,

    pub market_group: Option<i32>,

    pub group_item_title: Option<String>,

    pub group_item_threshold: Option<String>,

    pub question_id: Option<String>,

    pub uma_end_date: Option<String>,

    pub enable_order_book: Option<bool>,

    pub order_price_min_tick_size: Option<f64>,

    pub order_min_size: Option<f64>,

    pub uma_resolution_status: Option<String>,

    pub curation_order: Option<i32>,

    pub volume_num: Option<f64>,

    pub liquidity_num: Option<f64>,

    pub end_date_iso: Option<String>,

    pub start_date_iso: Option<String>,

    pub uma_end_date_iso: Option<String>,

    pub has_reviewed_dates: Option<bool>,

    pub ready_for_cron: Option<bool>,

    pub comments_enabled: Option<bool>,

    pub volume24hr: Option<f64>,
    pub volume1wk: Option<f64>,
    pub volume1mo: Option<f64>,
    pub volume1yr: Option<f64>,

    pub game_start_time: Option<String>,

    pub seconds_delay: Option<i32>,

    pub clob_token_ids: Option<String>,

    pub disqus_thread: Option<String>,

    pub short_outcomes: Option<String>,

    pub team_a_id: Option<String>,

    pub team_b_id: Option<String>,

    pub uma_bond: Option<String>,

    pub uma_reward: Option<String>,

    pub fpmm_live: Option<bool>,

    pub volume_24hr_amm: Option<f64>,

    pub volume_1mo_amm: Option<f64>,

    pub volume_1yr_amm: Option<f64>,

    pub volume_24hr_clob: Option<f64>,

    pub volume_1wk_clob: Option<f64>,

    pub volume_1mo_clob: Option<f64>,

    pub volume_1yr_clob: Option<f64>,

    pub volume_amm: Option<f64>,

    pub volume_clob: Option<f64>,

    pub liquidity_amm: Option<f64>,

    pub liquidity_clob: Option<f64>,

    pub maker_base_fee: Option<i32>,

    pub taker_base_fee: Option<i32>,

    pub custom_liveness: Option<i32>,

    pub accepting_orders: Option<bool>,

    pub notifications_enabled: Option<bool>,

    pub score: Option<i32>,

    pub image_optimized: Option<ImageOptimized>,

    pub icon_optimized: Option<ImageOptimized>,

    pub events: Option<Vec<Event>>,
    pub categories: Option<Vec<Category>>,
    pub tags: Option<Vec<Tag>>,
    pub creator: Option<String>,
    pub ready: Option<bool>,
    pub funded: Option<bool>,

    pub past_slugs: Option<String>,

    pub ready_timestamp: Option<String>,

    pub funded_timestamp: Option<String>,

    pub accepting_orders_timestamp: Option<String>,

    pub competitive: Option<f64>,

    pub rewards_min_size: Option<f64>,

    pub rewards_max_spread: Option<f64>,

    pub spread: Option<f64>,

    pub automatically_resolved: Option<bool>,

    pub one_day_price_change: Option<f64>,

    pub one_hour_price_change: Option<f64>,

    pub one_week_price_change: Option<f64>,

    pub one_month_price_change: Option<f64>,

    pub one_year_price_change: Option<f64>,

    pub last_trade_price: Option<f64>,

    pub best_bid: Option<f64>,

    pub best_ask: Option<f64>,

    pub automatically_active: Option<bool>,

    pub clear_book_on_start: Option<bool>,

    pub chart_color: Option<String>,

    pub series_color: Option<String>,

    pub show_gmp_series: Option<bool>,

    pub show_gmp_outcome: Option<bool>,

    pub manual_activation: Option<bool>,

    pub neg_risk_other: Option<bool>,

    pub game_id: Option<String>,

    pub group_item_range: Option<String>,

    pub sports_market_type: Option<String>,

    pub line: Option<f64>,

    pub uma_resolution_statuses: Option<String>,

    pub pending_deployment: Option<bool>,

    pub deploying: Option<bool>,

    pub deploying_timestamp: Option<String>,

    pub scheduled_deployment_timestamp: Option<String>,

    pub rfq_enabled: Option<bool>,

    pub event_start_time: Option<String>,
}

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
