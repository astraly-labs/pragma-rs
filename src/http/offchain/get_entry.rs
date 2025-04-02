use serde::{Deserialize, Serialize};

use crate::{client::PragmaClient, PragmaError};

/// Optional query parameters for the "Data Pair" endpoint.
#[derive(Debug, Default)]
pub struct GetEntryParams {
    pub timestamp: Option<u64>,
    pub interval: Option<String>,
    pub routing: Option<bool>,
    pub aggregation: Option<String>,
    pub entry_type: Option<String>,
    pub expiry: Option<String>,
    pub with_components: Option<bool>,
}

/// Individual price component from a source.
#[derive(Debug, Deserialize, Serialize)]
pub struct Component {
    pub price: String,
    pub source: String,
    pub timestamp: u64,
}

/// Response for the "Data Pair" offchain endpoint.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetEntryResponse {
    pub decimals: u32,
    pub num_sources_aggregated: u32,
    pub pair_id: String,
    pub price: String,
    pub timestamp: u64,
    #[serde(default)]
    pub components: Option<Vec<Component>>,
}

impl PragmaClient {
    /// Fetches price data for a trading pair from the offchain "Data Pair" endpoint.
    ///
    /// # Arguments
    ///
    /// * `base` - The base asset symbol (e.g., "BTC").
    /// * `quote` - The quote asset symbol (e.g., "USD").
    /// * `params` - Optional query parameters.
    ///
    /// # Examples
    ///
    /// ```
    /// use pragma_sdk::{Config, Environment, PragmaError, PragmaClient, GetEntryParams};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), PragmaError> {
    ///     let config = Config::new("your_api_key".to_string(), Environment::Development);
    ///     let client = PragmaClient::new(config)?;
    ///     let response = client.get_entry("BTC", "USD", None).await?;
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
        let url = format!("{}/data/{}/{}", self.config.base_url, base, quote);
        let mut request = self.http_client.get(&url);

        if let Some(p) = params {
            let mut query = Vec::new();
            if let Some(ts) = p.timestamp {
                query.push(("timestamp", ts.to_string()));
            }
            if let Some(interval) = p.interval {
                query.push(("interval", interval));
            }
            if let Some(routing) = p.routing {
                query.push(("routing", routing.to_string()));
            }
            if let Some(agg) = p.aggregation {
                query.push(("aggregation", agg));
            }
            if let Some(entry) = p.entry_type {
                query.push(("entry_type", entry));
            }
            if let Some(expiry) = p.expiry {
                query.push(("expiry", expiry));
            }
            if let Some(wc) = p.with_components {
                query.push(("with_components", wc.to_string()));
            }
            request = request.query(&query);
        }

        let response = request.send().await?.json::<GetEntryResponse>().await?;
        Ok(response)
    }
}
