use pragma_rs::{
    AggregationMode, Environment, GetEntryParams, InstrumentType, Interval, PragmaClient,
};

#[tokio::main]
async fn main() {
    let api_key = "MY_API_KEY".to_string();

    let config = pragma_rs::Config::new(api_key, Environment::Development);
    let client = PragmaClient::new(config).unwrap();

    assert!(client.is_healthy().await);

    let r = client
        .get_entry(
            "BTC",
            "USD",
            Some(GetEntryParams {
                timestamp: None,
                interval: Some(Interval::OneMinute),
                routing: Some(false),
                aggregation: Some(AggregationMode::Median),
                entry_type: Some(InstrumentType::Perp),
                with_components: Some(false),
            }),
        )
        .await
        .unwrap();
    println!("BTC/USD data:\n{r:?}");

    let r = client
        .get_historical_funding_rates("BTC", "USD", 1746448809, 1746535238, "hyperliquid")
        .await
        .unwrap();
    println!("BTC/USD historical funding rates:\n{r:?}");

    let r = client
        .get_funding_rates("BTC", "USD", "hyperliquid", None)
        .await
        .unwrap();
    println!("BTC/USD funding rates:\n{r:?}");
}
