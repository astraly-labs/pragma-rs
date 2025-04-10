use serde::{Deserialize, Serialize};

use pragma_common::{aggregation::AggregationMode, network::Network};

use crate::{PragmaClient, PragmaError};

/// Parameters for the `get_onchain_entry` method.
#[derive(Debug, Default)]
pub struct GetOnchainEntryParams {
    /// The network to query (required).
    pub network: Network,
    /// Optional aggregation mode.
    pub aggregation: Option<AggregationMode>,
    /// Optional routing flag.
    pub routing: Option<bool>,
    /// Optional timestamp filter.
    pub timestamp: Option<u64>,
    /// Whether to include components in the response.
    pub components: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OnchainComponent {
    /// The price from this component.
    pub price: String,
    /// The publisher of the data.
    pub publisher: String,
    /// The source of the data.
    pub source: String,
    /// The timestamp of the data.
    pub timestamp: u64,
    /// The transaction hash.
    pub tx_hash: String,
}

/// Response from the `get_onchain_entry` method.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetOnchainEntryResponse {
    /// The type of the asset.
    pub asset_type: String,
    /// The number of decimal places for the price.
    pub decimals: u32,
    /// The timestamp of the last update.
    pub last_updated_timestamp: u64,
    /// The number of sources aggregated.
    pub nb_sources_aggregated: u32,
    /// The identifier of the trading pair.
    pub pair_id: String,
    /// The aggregated price.
    pub price: String,
    /// Optional list of component data.
    #[serde(default)]
    pub components: Option<Vec<OnchainComponent>>,
}

impl PragmaClient {
    /// Fetches onchain entry data for a trading pair.
    ///
    /// # Arguments
    ///
    /// * `base` - The base asset symbol (e.g., "BTC").
    /// * `quote` - The quote asset symbol (e.g., "USD").
    /// * `params` - The parameters for the query, including the required `network`.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `GetOnchainEntryResponse` on success, or a `PragmaError` on failure.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use your_crate_name::{PragmaClient, GetOnchainEntryParams, Network, AggregationMode};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let config = Config::new("your_api_key".to_string(), Environment::Development);
    ///     let client = PragmaClient::new(config)?;
    ///     let params = GetOnchainEntryParams {
    ///         network: Network::Mainnet,
    ///         aggregation: Some(AggregationMode::Median),
    ///         components: Some(true),
    ///         routing: None,
    ///         timestamp: None,
    ///     };
    ///     let response = client.get_onchain_entry("BTC", "USD", params).await?;
    ///     println!("Price: {}", response.price);
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_onchain_entry(
        &self,
        base: &str,
        quote: &str,
        params: GetOnchainEntryParams,
    ) -> Result<GetOnchainEntryResponse, PragmaError> {
        let url = format!(
            "{}/node/v1/onchain/{}/{}",
            self.config.base_url, base, quote
        );
        let mut request = self.http_client.get(&url);

        let mut query = vec![("network", params.network.as_str().to_string())];

        if let Some(agg) = params.aggregation {
            query.push(("aggregation", agg.as_str().to_string()));
        }
        if let Some(routing) = params.routing {
            query.push(("routing", routing.to_string()));
        }
        if let Some(ts) = params.timestamp {
            query.push(("timestamp", ts.to_string()));
        }
        if let Some(comps) = params.components {
            query.push(("components", comps.to_string()));
        }

        request = request.query(&query);

        let response = request
            .send()
            .await?
            .json::<GetOnchainEntryResponse>()
            .await?;
        Ok(response)
    }

    #[cfg(feature = "sync")]
    /// Sync version of `get_onchain_entry`
    pub fn get_onchain_entry_sync(
        &self,
        base: &str,
        quote: &str,
        params: GetOnchainEntryParams,
    ) -> Result<GetOnchainEntryResponse, PragmaError> {
        let url = format!(
            "{}/node/v1/onchain/{}/{}",
            self.config.base_url, base, quote
        );
        let mut request = self.http_blocking_client.get(&url);

        let mut query = vec![("network", params.network.as_str().to_string())];
        if let Some(agg) = params.aggregation {
            query.push(("aggregation", agg.as_str().to_string()));
        }
        if let Some(routing) = params.routing {
            query.push(("routing", routing.to_string()));
        }
        if let Some(ts) = params.timestamp {
            query.push(("timestamp", ts.to_string()));
        }
        if let Some(comps) = params.components {
            query.push(("components", comps.to_string()));
        }
        request = request.query(&query);

        let response = request.send()?;

        match response.status() {
            reqwest::StatusCode::UNAUTHORIZED => {
                let text = response
                    .text()
                    .unwrap_or_else(|_| "No additional details".to_string());
                return Err(PragmaError::Unauthorized(text));
            }
            status if !status.is_success() => {
                let text = response
                    .text()
                    .unwrap_or_else(|_| "Unknown error".to_string());
                return Err(PragmaError::ApiError(format!(
                    "API returned status {}: {}",
                    status, text
                )));
            }
            _ => {}
        }

        response
            .json::<GetOnchainEntryResponse>()
            .map_err(PragmaError::HttpError)
    }
}
