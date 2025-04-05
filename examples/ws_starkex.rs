use std::time::Duration;

use pragma_rs::{Config, Environment, PragmaClient, StarkexMessage};

#[tokio::main]
async fn main() {
    let api_key = "MY_API_KEY".to_string();

    let config = Config::new(api_key, Environment::Development);
    let client = PragmaClient::new(config).unwrap();

    let mut ws_client = client.starkex_ws_client();
    ws_client.connect().await.unwrap();

    ws_client
        .send(StarkexMessage::Subscribe {
            msg_type: "subscribe".into(),
            pairs: vec!["BTC/USD".to_string(), "ETH/USD".to_string()],
        })
        .await
        .unwrap();

    tokio::spawn(async move {
        while let Some(msg) = ws_client.next_message().await {
            match msg {
                StarkexMessage::PriceUpdate {
                    oracle_prices,
                    timestamp,
                } => {
                    println!("[{timestamp}] {oracle_prices:?}");
                }
                StarkexMessage::Subscribe { msg_type, pairs } => {
                    println!("{msg_type} to {pairs:?}");
                }
                _ => {}
            }
        }
    });

    tokio::time::sleep(Duration::from_secs(20)).await;
}
