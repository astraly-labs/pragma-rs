mod client;
mod config;
mod errors;
mod http;
mod types;
mod ws;

pub use client::PragmaClient;
pub use config::{Config, Environment};
pub use errors::PragmaError;

// Types
pub use types::{aggregation::AggregationMode, interval::Interval};

// Offchain endpoints
pub use http::offchain::get_entry::{Component, GetEntryParams, GetEntryResponse};

// Onchain endpoints
pub use http::onchain::get_onchain_entry::{
    GetOnchainEntryParams, GetOnchainEntryResponse, OnchainComponent,
};

// Websocket endpoints
pub use ws::{
    lightspeed::{LightspeedMessage, PriceUpdate},
    starkex::{PriceUpdate as StarkexPriceUpdate, SignedPrice, StarkexMessage},
};
