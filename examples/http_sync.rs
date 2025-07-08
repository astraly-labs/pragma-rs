use pragma_rs::{
    AggregationMode, Environment, GetEntryParams, InstrumentType, Interval, PragmaClient,
};

fn main() {
    let api_key = "MY_API_KEY".to_string();

    let config = pragma_rs::Config::new(api_key, Environment::Development);
    let client = PragmaClient::new(config).unwrap();

    assert!(client.is_healthy_sync());

    let r = client
        .get_entry_sync(
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
        .unwrap();

    println!("BTC/USD data:\n{r:?}");

    let r = client
        .get_historical_funding_rates_sync("BTC", "USD", 1746448809, 1746535238, "hyperliquid")
        .unwrap();
    println!("BTC/USD historical funding rates:\n{r:?}");

    let r = client
        .get_funding_rates_sync("BTC", "USD", "hyperliquid", None)
        .unwrap();
    println!("BTC/USD funding rates:\n{r:?}");
}
