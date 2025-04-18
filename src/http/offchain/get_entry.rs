use std::num::ParseIntError;

#[cfg(feature = "bigdecimal")]
use bigdecimal::BigDecimal;
use pragma_common::{
    aggregation::AggregationMode, instrument_type::InstrumentType, interval::Interval,
};
use serde::{Deserialize, Serialize};

use crate::{client::PragmaClient, PragmaError};

/// Optional query parameters for the "Data Pair" endpoint.
///
/// This struct defines the parameters that can be passed to customize data retrieval.
#[derive(Debug, Default)]
pub struct GetEntryParams {
    /// The timestamp for which to retrieve data, in milliseconds since the Unix epoch.
    pub timestamp: Option<u64>,

    /// The time interval over which to aggregate data.
    pub interval: Option<Interval>,

    /// Whether to enable routing for the query.
    pub routing: Option<bool>,

    /// The aggregation mode to apply to the data.
    pub aggregation: Option<AggregationMode>,

    /// The type of entry to retrieve (e.g., spot or perp).
    pub entry_type: Option<InstrumentType>,

    /// Whether to include component data in the response.
    pub with_components: Option<bool>,
}

/// Individual price component from a source.
///
/// Represents a single price contribution from a specific source.
#[derive(Debug, Deserialize, Serialize)]
pub struct Component {
    /// The price value as a string.
    pub price: String,

    /// The source of the price data.
    pub source: String,

    /// The timestamp of the price data, in milliseconds since the Unix epoch.
    pub timestamp: u64,
}

/// Response for the "Data Pair" offchain endpoint.
///
/// Contains the aggregated price data and optional components for a trading pair.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetEntryResponse {
    /// The number of decimal places in the price.
    pub decimals: u32,

    /// The number of sources aggregated to compute the price.
    pub num_sources_aggregated: u32,

    /// The identifier of the trading pair (e.g., "BTC/USD").
    pub pair_id: String,

    /// The aggregated price as a string.
    pub price: String,

    /// The timestamp of the price data, in milliseconds since the Unix epoch.
    pub timestamp: u64,

    /// Optional list of individual price components, included if `with_components` is true.
    #[serde(default)]
    pub components: Option<Vec<Component>>,
}

impl GetEntryResponse {
    pub fn price_u128(&self) -> Result<u128, ParseIntError> {
        u128::from_str_radix(&self.price.replace("0x", ""), 16)
    }

    #[cfg(feature = "bigdecimal")]
    pub fn price_bd(&self) -> Result<BigDecimal, ParseIntError> {
        let price_u128 = u128::from_str_radix(&self.price.replace("0x", ""), 16)?;
        Ok(BigDecimal::new(price_u128.into(), i64::from(self.decimals)))
    }
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
    pub async fn get_entry(
        &self,
        base: &str,
        quote: &str,
        params: Option<GetEntryParams>,
    ) -> Result<GetEntryResponse, PragmaError> {
        let url = format!("{}/node/v1/data/{}/{}", self.config.base_url, base, quote);
        let mut request = self.http_client.get(&url);

        if let Some(p) = params {
            let mut query = Vec::new();
            if let Some(ts) = p.timestamp {
                query.push(("timestamp", ts.to_string()));
            }
            if let Some(interval) = p.interval {
                query.push(("interval", interval.as_str().to_string()));
            }
            if let Some(routing) = p.routing {
                query.push(("routing", routing.to_string()));
            }
            if let Some(agg) = p.aggregation {
                query.push(("aggregation", agg.as_str().to_string()));
            }
            if let Some(entry) = p.entry_type {
                query.push(("entry_type", entry.to_string()));
            }
            if let Some(wc) = p.with_components {
                query.push(("with_components", wc.to_string()));
            }
            request = request.query(&query);
        }

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
            .json::<GetEntryResponse>()
            .await
            .map_err(PragmaError::HttpError)
    }

    #[cfg(feature = "sync")]
    pub fn get_entry_sync(
        &self,
        base: &str,
        quote: &str,
        params: Option<GetEntryParams>,
    ) -> Result<GetEntryResponse, PragmaError> {
        let runtime = Self::runtime();
        runtime.block_on(self.get_entry(base, quote, params))
    }
}
