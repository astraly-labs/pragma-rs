use crate::PragmaClient;
use serde::{Deserialize, Serialize};

use super::PragmaWsClient;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LightspeedMessage {
    Subscribe {
        msg_type: String,
        pairs: Vec<String>,
    },
    Unsubscribe {
        msg_type: String,
        pairs: Vec<String>,
    },
    PriceUpdate {
        oracle_prices: Vec<PriceUpdate>,
        timestamp: u32,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceUpdate {
    /// The unique identifier for the asset pair (e.g., "0x12345").
    pub pair_id: String,

    /// The aggregated price of the asset pair, returned as a string to preserve precision.
    pub price: String,

    /// The number of sources aggregated to compute the price.
    pub num_sources_aggregated: String,
}

impl PragmaClient {
    /// Creates a WebSocket client for the Lightspeed endpoint.
    ///
    /// This method configures a `PragmaWsClient` to connect to the Lightspeed WebSocket endpoint,
    /// which provides ultra-fast price updates every 500ms without verification metadata.
    pub fn lightspeed_ws_client(&self) -> PragmaWsClient<LightspeedMessage> {
        let url = format!("{}/data/price/subscribe", self.config.ws_url);
        PragmaWsClient::new(url, |msg| {
            serde_json::from_str::<LightspeedMessage>(&msg).ok()
        })
    }
}
