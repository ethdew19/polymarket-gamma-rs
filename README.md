# polymarket-gamma-rs

A Rust wrapper for the Polymarket Gamma API, providing read-only access to public prediction market data. Query and search markets, events, and comments without requiring authentication.

## Features

- **Markets**: Query and filter prediction markets
- **Events**: Access event information and details
- **Comments**: Retrieve market comments and discussions
- **Search**: Search across markets, events and profiles
- **Series**: Get information about market series
- **Sports**: Access sports market meta data
- **Tags**: Query markets by tags and categories

**Note**: This wrapper provides read-only access to Polymarket's Gamma API. It does not support trading, placing orders, or account management. For trading functionality, you'll need to use Polymarket's CLOB API separately. For access to the polymarket Data api see https://github.com/ethdew19/polymarket-data-rs.

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
polymarket-gamma-rs = { git = "https://github.com/ethdew19/polymarket-gamma-rs" }
tokio = { version = "1", features = ["full"] }
```

Example Usage:

```rust
use polymarket_gamma_rs::{GammaClient, RestError, types_markets::ListMarketsArgs};

#[tokio::main]
async fn main() -> Result<(), RestError> {
    // Create a new client
    let client = GammaClient::default();

    // List markets with default parameters
    let args = ListMarketsArgs::default();
    let markets = client.list_markets(args).await?;

    println!("Found {} markets", markets.len());

    Ok(())
}
```

## License

Licensed under the MIT License. See [LICENSE](LICENSE) for details.
