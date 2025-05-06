use crate::{PragmaClient, PragmaError};

use super::FundingRatesEntry;

pub type GetFundingRatesResponse = FundingRatesEntry;

impl PragmaClient {
    /// Fetches funding rate data for a trading pair from the offchain "Funding Rates" endpoint.
    ///
    /// This method retrieves funding rate data for a specified base and quote asset pair on a specific source.
    /// if a timestamp is provided, the last funding rate data before that timestamp will be returned.
    /// # Arguments
    ///
    /// * `base` - The base asset symbol (e.g., "BTC").
    /// * `quote` - The quote asset symbol (e.g., "USD").
    /// * `source` - The source of the funding rate data.
    /// * `timestamp_s` - Optional timestamp in seconds since the Unix epoch.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `GetFundingRatesResponse` on success, or a `PragmaError` on failure.
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
    ///     let response = client.get_funding_rates("BTC", "USD", "hyperliquid", None).await?;
    ///     println!("Funding Rate: {}", response.hourly_rate);
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


