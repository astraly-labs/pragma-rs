use std::num::ParseIntError;

#[cfg(feature = "bigdecimal")]
use bigdecimal::BigDecimal;
use pragma_common::{
    aggregation::AggregationMode, instrument_type::InstrumentType, interval::Interval,
};
use serde::{Deserialize, Serialize};

use crate::{PragmaClient, PragmaError};

/// Response for the "Data Pair" offchain endpoint.
///
/// Contains the aggregated price data and optional components for a trading pair.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetFundingRatesResponse {
    /// The number of decimal places in the price.
    pub hourly_rate: f64,
    
    /// The identifier of the trading pair (e.g., "BTC/USD").
    pub pair: String,

    /// The aggregated price as a string.
    pub source: String,

    /// The timestamp of the price data, in milliseconds since the Unix epoch.
    pub timestamp_ms: u64,
}

impl PragmaClient {
    /// Fetches price data for a trading pair from the offchain "Data Pair" endpoint.
    ///
    /// This method retrieves price data for a specified base and quote asset pair, with optional parameters to customize
    /// the aggregation and filtering of the data.
    ///
    /// # Arguments
    ///
    /// * `base` - The base asset symbol (e.g., "BTC").
    /// * `quote` - The quote asset symbol (e.g., "USD").
    /// * `params` - Optional query parameters to customize the request.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `GetEntryResponse` on success, or a `PragmaError` on failure.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use pragma_rs::{Config, Environment, PragmaError, PragmaClient, GetEntryParams, Interval, AggregationMode};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), PragmaError> {
    ///     let config = Config::new("your_api_key".to_string(), Environment::Development);
    ///     let client = PragmaClient::new(config)?;
    ///     let params = GetEntryParams {
    ///         interval: Some(Interval::OneHour),
    ///         aggregation: Some(AggregationMode::Median),
    ///         with_components: Some(true),
    ///         ..Default::default()
    ///     };
    ///     let response = client.get_entry("BTC", "USD", Some(params)).await?;
    ///     println!("Price: {}", response.price);
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_funding_rates(
        &self,
        base: &str,
        quote: &str,
        source: &str,
        timestamp_s: Option<u64>,
    ) -> Result<GetFundingRatesResponse, PragmaError> {
        let url = format!("{}/node/v1/funding_rates/{}/{}", self.config.base_url, base, quote);
        let mut query = vec![
            ("source", source.to_string()),
            ];
        if let Some(timestamp_s) = timestamp_s {
            query.push(("timestamp", timestamp_s.to_string()));
        }
        let request = self.http_client.get(&url).query(&query);

        let response = request.send().await?;

        match response.status() {
            reqwest::StatusCode::UNAUTHORIZED => {
                let text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "No additional details".to_string());
                return Err(PragmaError::Unauthorized(text));
            }
            status if !status.is_success() => {
                let text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                return Err(PragmaError::ApiError(format!(
                    "API returned status {status}: {text}",
                )));
            }
            _ => {}
        }

        response
            .json::<GetFundingRatesResponse>()
            .await
            .map_err(PragmaError::HttpError)
    }

    #[cfg(feature = "sync")]
    pub fn get_funding_rates_sync(
        &self,
        base: &str,
        quote: &str,
        source: &str,
        timestamp_s: Option<u64>,
    ) -> Result<GetFundingRatesResponse, PragmaError> {
        let runtime = Self::runtime();
        runtime.block_on(self.get_funding_rates(base, quote, source, timestamp_s))
    }
}


