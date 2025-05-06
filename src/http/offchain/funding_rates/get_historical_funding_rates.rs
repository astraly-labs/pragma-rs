use crate::{PragmaClient, PragmaError};

use super::FundingRatesEntry;

pub type GetHistoricalFundingRatesResponse = Vec<FundingRatesEntry>;

impl PragmaClient {
    /// Fetches historical funding rate data for a trading pair from the offchain "Historical Funding Rates" endpoint.
    ///
    /// This method retrieves historical funding rate data for a specified base and quote asset pair on a specific source.
    ///
    /// # Arguments
    ///
    /// * `base` - The base asset symbol (e.g., "BTC").
    /// * `quote` - The quote asset symbol (e.g., "USD").
    /// * `from_ts` - The start timestamp in milliseconds since the Unix epoch.
    /// * `to_ts` - The end timestamp in milliseconds since the Unix epoch.
    /// * `source` - The source of the funding rate data.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `GetHistoricalFundingRatesResponse` on success, or a `PragmaError` on failure.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use pragma_rs::{Config, Environment, PragmaError, PragmaClient};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), PragmaError> {
    ///     let config = Config::new("your_api_key".to_string(), Environment::Development);
    ///     let client = PragmaClient::new(config)?;
    ///     let response = client.get_historical_funding_rates("BTC", "USD", 1746448809, 1746535238, "hyperliquid").await?;
    ///     println!("Historical Funding Rates: {}", response.hourly_rate);
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_historical_funding_rates(
        &self,
        base: &str,
        quote: &str,
        from_ts: u128,
        to_ts: u128,
        source: &str,
    ) -> Result<GetHistoricalFundingRatesResponse, PragmaError> {
        let url = format!("{}/node/v1/funding_rates/history/{}/{}", self.config.base_url, base, quote);
        let query = vec![
            ("timestamp", format!("{},{}", from_ts, to_ts)),
            ("source", source.to_string()),
        ];
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
            .json::<GetHistoricalFundingRatesResponse>()
            .await
            .map_err(PragmaError::HttpError)
    }

    #[cfg(feature = "sync")]
    pub fn get_historical_funding_rates_sync(
        &self,
        base: &str,
        quote: &str,
        from_ts: u128,
        to_ts: u128,
        source: &str,
    ) -> Result<GetHistoricalFundingRatesResponse, PragmaError> {
        let runtime = Self::runtime();
        runtime.block_on(self.get_historical_funding_rates(base, quote, from_ts, to_ts, source))
    }
}


