use serde::{Deserialize, Serialize};

pub mod get_funding_rates;
pub mod get_historical_funding_rates;

pub use get_funding_rates::GetFundingRatesResponse;
pub use get_historical_funding_rates::GetHistoricalFundingRatesResponse;

// Response for the "Historical Funding Rates" offchain endpoint.
///
/// Contains the historical funding rate data for a trading pair on a specific source.
#[derive(Debug, Deserialize, Serialize)]
pub struct FundingRatesEntry {
    /// The hourly funding rate as a percentage.
    pub hourly_rate: f64,
    
    /// The identifier of the trading pair (e.g., "BTC/USD").
    pub pair: String,

    /// The source of the funding rate data.
    pub source: String,

    /// The timestamp of the funding rate data, in milliseconds since the Unix epoch.
    pub timestamp_ms: u64,
}
