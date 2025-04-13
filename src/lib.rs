mod client;
mod config;
mod errors;
mod http;
mod ws;

#[cfg(feature = "sync")]
use std::{cell::RefCell, thread_local};

#[cfg(feature = "sync")]
thread_local! {
    pub(crate) static BLOCKING_CLIENT: RefCell<Option<reqwest::blocking::Client>> = const { RefCell::new(None) };
}

pub use client::PragmaClient;
pub use config::{Config, Environment};
pub use errors::PragmaError;

// Re-export types from pragma_common
pub use pragma_common::{
    aggregation::AggregationMode, instrument_type::InstrumentType, interval::Interval,
    web3::StarknetNetwork,
};

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
