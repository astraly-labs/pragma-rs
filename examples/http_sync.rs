use pragma_rs::{
    AggregationMode, Environment, GetEntryParams, InstrumentType, Interval, PragmaClient,
};

fn main() {
    let api_key = "nOCBKDhmCOxhIYjzsdf1MgasQQCGBB7d".to_string();

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
}
