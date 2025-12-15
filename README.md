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

**Note**: This wrapper provides read-only access to Polymarket's public data via the Gamma API. It does not support trading, placing orders, or account management. For trading functionality, you'll need to use Polymarket's CLOB API separately.

## Quick Start

```rust
use polymarket_gamma_rs::{GammaClient, ListMarketsArgs, RestError};

#[tokio::main]
async fn main() -> Result<(), RestError>> {
    // Create a new client
    let client = GammaClient::new();

    // List markets with default parameters
    let args = ListMarketsArgs::default();
    let markets = client.list_markets(args).await?;

    println!("Found {} markets", markets.len());

    Ok(())
}
```

## License

Licensed under the MIT License. See [LICENSE](LICENSE) for details.
