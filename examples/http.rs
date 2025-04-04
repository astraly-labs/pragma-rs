use pragma_rs::{AggregationMode, Environment, GetEntryParams, Interval, PragmaClient};

#[tokio::main]
async fn main() {
    let api_key = "MY_API_KEY".to_string();

    let config = pragma_rs::Config::new(api_key, Environment::Development);
    let client = PragmaClient::new(config).unwrap();

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
    println!("BTC/USD data:\n{r:?}");
}
