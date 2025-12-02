use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize)]
pub struct ListEventsArgs {
    /// Range: x >= 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Range: x >= 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,

    /// Comma-separated list of fields to order by
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ascending: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Vec<i32>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_tag_id: Option<Vec<i32>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_tags: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub featured: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cyom: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_chat: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_template: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date_min: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date_max: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date_min: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date_max: Option<String>,
}

pub type ListEventsResponse = Vec<Event>;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub id: String,
    pub ticker: Option<String>,
    pub slug: Option<String>,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub description: Option<String>,

    pub resolution_source: Option<String>,

    pub start_date: Option<String>,

    pub creation_date: Option<String>,

    pub end_date: Option<String>,

    pub image: Option<String>,
    pub icon: Option<String>,
    pub active: Option<bool>,
    pub closed: Option<bool>,
    pub archived: Option<bool>,
    pub new: Option<bool>,
    pub featured: Option<bool>,
    pub restricted: Option<bool>,
    pub liquidity: Option<f64>,
    pub volume: Option<f64>,

    pub open_interest: Option<f64>,

    pub sort_by: Option<String>,

    pub category: Option<String>,
    pub subcategory: Option<String>,

    pub is_template: Option<bool>,

    pub template_variables: Option<String>,

    pub published_at: Option<String>,

    pub created_by: Option<String>,

    pub updated_by: Option<String>,

    pub created_at: Option<String>,

    pub updated_at: Option<String>,

    pub comments_enabled: Option<bool>,

    pub competitive: Option<f64>,
    pub volume24hr: Option<f64>,
    pub volume1wk: Option<f64>,
    pub volume1mo: Option<f64>,
    pub volume1yr: Option<f64>,

    pub featured_image: Option<String>,

    pub disqus_thread: Option<String>,

    pub parent_event: Option<String>,

    pub enable_order_book: Option<bool>,

    pub liquidity_amm: Option<f64>,

    pub liquidity_clob: Option<f64>,

    pub neg_risk: Option<bool>,

    pub neg_risk_market_id: Option<String>,

    pub neg_risk_fee_bips: Option<i32>,

    pub comment_count: Option<i32>,

    pub image_optimized: Option<ImageOptimized>,

    pub icon_optimized: Option<ImageOptimized>,

    pub featured_image_optimized: Option<ImageOptimized>,

    pub sub_events: Option<Vec<String>>,

    pub markets: Option<Vec<Market>>,
    pub categories: Option<Vec<Category>>,
    pub collections: Option<Vec<Collection>>,
    pub series: Option<Vec<Series>>,
    pub chats: Option<Vec<Chat>>,

    pub event_creators: Option<Vec<EventCreator>>,

    pub tweet_count: Option<i32>,

    pub featured_order: Option<i32>,

    pub estimate_value: Option<bool>,

    pub cant_estimate: Option<bool>,

    pub estimated_value: Option<String>,

    pub templates: Option<Vec<Template>>,

    pub spreads_main_line: Option<f64>,

    pub totals_main_line: Option<f64>,

    pub carousel_map: Option<String>,

    pub pending_deployment: Option<bool>,

    pub deploying: Option<bool>,

    pub deploying_timestamp: Option<String>,

    pub scheduled_deployment_timestamp: Option<String>,

    pub game_status: Option<String>,
}

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

#[derive(Debug, Clone, Deserialize)]
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

#[derive(Debug, Clone, Deserialize)]
pub struct Collection {
    pub id: String,
    pub ticker: Option<String>,
    pub slug: Option<String>,
    pub title: Option<String>,
    pub subtitle: Option<String>,

    pub collection_type: Option<String>,

    pub description: Option<String>,
    pub tags: Option<String>,
    pub image: Option<String>,
    pub icon: Option<String>,

    pub header_image: Option<String>,

    pub layout: Option<String>,
    pub active: Option<bool>,
    pub closed: Option<bool>,
    pub archived: Option<bool>,
    pub new: Option<bool>,
    pub featured: Option<bool>,
    pub restricted: Option<bool>,

    pub is_template: Option<bool>,

    pub template_variables: Option<String>,

    pub published_at: Option<String>,

    pub created_by: Option<String>,

    pub updated_by: Option<String>,

    pub created_at: Option<String>,

    pub updated_at: Option<String>,

    pub comments_enabled: Option<bool>,

    pub image_optimized: Option<ImageOptimized>,

    pub icon_optimized: Option<ImageOptimized>,

    pub header_image_optimized: Option<ImageOptimized>,

    pub tags_list: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub id: String,
    pub label: Option<String>,
    pub parent_category: Option<String>,
    pub slug: Option<String>,
    pub published_at: Option<String>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
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

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chat {
    pub id: String,
    pub channel_id: Option<String>,
    pub channel_name: Option<String>,
    pub channel_image: Option<String>,
    pub live: Option<bool>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventCreator {
    pub id: String,
    pub creator_name: Option<String>,
    pub creator_handle: Option<String>,
    pub creator_url: Option<String>,
    pub creator_image: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Template {
    pub id: String,
    pub event_title: Option<String>,
    pub event_slug: Option<String>,
    pub event_image: Option<String>,
    pub market_title: Option<String>,
    pub description: Option<String>,
    pub resolution_source: Option<String>,
    pub neg_risk: Option<bool>,
    pub sort_by: Option<String>,
    pub show_market_images: Option<bool>,
    pub series_slug: Option<String>,
    pub outcomes: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageOptimized {
    pub id: String,
    pub image_url_source: Option<String>,
    pub image_url_optimized: Option<String>,
    pub image_size_kb_source: Option<u64>,
    pub image_size_kb_optimized: Option<u64>,
    pub image_optimized_complete: Option<bool>,
    pub image_optimized_last_updated: Option<String>,
    pub rel_id: Option<u64>,
    pub field: Option<String>,
    pub relname: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEventByIdArgs {
    #[serde(skip_serializing)]
    pub id: u64,
    pub include_chat: Option<bool>,
    pub include_template: Option<bool>,
}

pub type GetEventByIdResponse = Event;

pub type GetEventTagsResponse = Vec<EventTags>;
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventTags {
    pub id: String,
    pub label: Option<String>,
    pub slug: Option<String>,
    pub force_show: Option<bool>,
    pub published_at: Option<String>,
    pub created_by: Option<u64>,
    pub updated_by: Option<u64>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub force_hide: Option<bool>,
    pub is_carousel: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEventBySlugArgs {
    #[serde(skip_serializing)] // this is used in the path so we dont need to serialize it
    pub slug: String,
    pub include_chat: bool,
    pub include_template: bool,
}

pub type GetEventBySlugResponse = Event;
