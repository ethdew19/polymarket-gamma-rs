pub mod client;
pub mod endpoints;
pub mod error;
pub mod types;

// Re-export core types at crate root
pub use client::GammaClient;
pub use error::{RestError, Result};

// Re-export type modules for convenience
pub use types::types_comments;
pub use types::types_events;
pub use types::types_markets;
pub use types::types_profiles;
pub use types::types_search;
pub use types::types_series;
pub use types::types_sports;
pub use types::types_tags;
