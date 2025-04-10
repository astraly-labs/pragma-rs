# pragma-rs 🦀

**pragma-rs** is a Rust SDK for the [Pragma API](https://docs.pragma.build/api-reference/introduction), making it easy to fetch offchain and onchain data or get real-time updates via WebSocket.

> [!WARNING]  
> This crate is still **in progress** and **not ready for production**

## 📦 Installation

Add `pragma-rs` to your Rust project by tossing this into your `Cargo.toml`:

```toml
[dependencies]
pragma-rs = "0.1.8"
```

🔧 **Tip**: You’ll need tokio since this SDK uses async/await.

You can also use the `sync` version by using the feature:

```toml
[dependencies]
pragma-rs = { version = "0.1.8", features = ["sync"] }
```

## 🚀 Quick Start

### 1. Set Up Your Config

Kick things off by setting up your API key and environment:

```rust
use pragma_rs::{Config, Environment};

let api_key = "MY_API_KEY".to_string();
let config = Config::new(api_key, Environment::Development);
```

##### 🌍 Environments:
* `Local`: Can be configured with any url.
* `Development`: Perfect for testing and tinkering.
* `Production`: For the real deal (not ready yet, stay tuned).

### 2. Create a Client

Spin up a PragmaClient with your config:

```rust
let client = PragmaClient::new(config).unwrap();
```

### 3. Fetch data using http

```rust
use pragma_rs::{AggregationMode, GetEntryParams, Interval, PragmaClient};

#[tokio::main]
async fn main() {
    let api_key = "MY_API_KEY".to_string();
    let config = Config::new(api_key, Environment::Development);
    let client = PragmaClient::new(config).unwrap();

    let r = client.get_entry("BTC", "USD", None).await.unwrap();
    println!("BTC/USD data:\n{r:?}");

    // Or with options
    let r = client
        .get_entry(
            "BTC",
            "USD",
            Some(GetEntryParams {
                timestamp: None,
                interval: Some(Interval::OneMinute),
                routing: Some(false),
                aggregation: Some(AggregationMode::Median),
                entry_type: None,
                with_components: Some(true),
            }),
        )
        .await
        .unwrap();
    println!("BTC/USD detailed data:\n{r:?}");
}
```

### 4. Or using the real-time websocket

```rust
use std::time::Duration;
use pragma_rs::{Config, Environment, LightspeedMessage, PragmaClient};

#[tokio::main]
async fn main() {
    let api_key = "MY_API_KEY".to_string();
    let config = Config::new(api_key, Environment::Development);
    let client = PragmaClient::new(config).unwrap();

    let mut ws_client = client.lightspeed_ws_client();
    ws_client.connect().await.unwrap();

    ws_client
        .send(LightspeedMessage::Subscribe {
            msg_type: "subscribe".into(),
            pairs: vec!["BTC/USD".to_string(), "ETH/USD".to_string()],
        })
        .await
        .unwrap();

    tokio::spawn(async move {
        while let Some(msg) = ws_client.next_message().await {
            match msg {
                LightspeedMessage::PriceUpdate {
                    oracle_prices,
                    timestamp,
                } => {
                    println!("[{timestamp}] {oracle_prices:?}");
                }
                LightspeedMessage::Subscribe { msg_type, pairs } => {
                    println!("{msg_type} to {pairs:?}");
                }
                _ => {}
            }
        }
    });

    // Hang out for 20 seconds to see the updates
    tokio::time::sleep(Duration::from_secs(20)).await;
}
```

## 🛠️ Contribute

We’re still building this crate, and we’d love your help! Here’s how you can chip in:

* 🐞 Found a bug? Open an issue.
* 💡 Got an idea? Suggest a feature.
* 🔧 Want to code? Send us a pull request!

## 📜 License

Licensed under the MIT License - see [here](./LICENSE) for details.
