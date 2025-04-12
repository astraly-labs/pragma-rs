use crate::PragmaClient;
use serde::{Deserialize, Serialize};

use super::PragmaWsClient;

/// Enum representing the possible messages for the Starkex WebSocket endpoint.
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StarkexMessage {
    Subscribe {
        #[serde(rename = "msg_type")]
        msg_type: String,
        pairs: Vec<String>,
    },
    Unsubscribe {
        #[serde(rename = "msg_type")]
        msg_type: String,
        pairs: Vec<String>,
    },
    PriceUpdate {
        oracle_prices: Vec<PriceUpdate>,
        timestamp: i64,
    },
}

/// Struct representing a price update from the Starkex endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct PriceUpdate {
    /// The global identifier for the asset.
    pub global_asset_id: String,

    /// The median price of the asset, returned as a string to preserve precision.
    pub median_price: String,

    /// The cryptographic signature for the price update.
    pub signature: String,

    /// A list of signed prices from individual oracles.
    pub signed_prices: Vec<SignedPrice>,
}

/// Struct representing a signed price from an individual oracle.
#[derive(Debug, Serialize, Deserialize)]
pub struct SignedPrice {
    /// The identifier for the oracle's asset.
    pub oracle_asset_id: String,

    /// The price reported by the oracle, returned as a string to preserve precision.
    pub oracle_price: String,

    /// The public key used for signing the price.
    pub signing_key: String,

    /// The timestamp of the price data.
    pub timestamp: i64,

    /// The cryptographic signature of the price data.
    pub signature: String,
}

impl PragmaClient {
    /// Creates a WebSocket client for the Starkex endpoint.
    ///
    /// This method configures a `PragmaWsClient` to connect to the Starkex WebSocket endpoint,
    /// which provides verifiable price updates with cryptographic signatures.
    pub fn starkex_ws_client(&self) -> PragmaWsClient<StarkexMessage> {
        let url = format!("{}/node/v1/data/subscribe", self.config.ws_url);
        let api_key = self.config.api_key.clone();
        PragmaWsClient::new(url, api_key, |msg| {
            if let Ok(msg) = serde_json::from_str::<String>(&msg) {
                serde_json::from_str::<StarkexMessage>(&msg).ok()
            } else {
                None
            }
        })
    }
}
