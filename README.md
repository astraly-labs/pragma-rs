# pragma-rs

🦀 Pragma API SDK for Rust.

## ⛑️ In progress! Not suited for Production.

### Smol Example

```rust
use pragma_rs::{Environment, GetEntryParams, PragmaClient};

#[tokio::main]
async fn main() {
    let api_key = "MY_API_KEY".to_string();

    let config = pragma_rs::Config::new(api_key, Environment::Development);
    let client = PragmaClient::new(config).unwrap();

    // Simple call
    let r = client.get_entry("BTC", "USD", None).await.unwrap();
    println!("BTC/USD:\n{r:?}");

    // Or with some options
    let r = client
        .get_entry(
            "ETH",
            "STRK",
            Some(GetEntryParams {
                timestamp: None,
                interval: None,
                routing: Some(true),
                aggregation: None,
                entry_type: None,
                with_components: Some(false),
                expiry: None,
            }),
        )
        .await
        .unwrap();
    println!("STRK/ETH:\n{r:?}");
}
```
