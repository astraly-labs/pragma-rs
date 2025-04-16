#[cfg(feature = "sync")]
use std::sync::Once;

use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
#[cfg(feature = "sync")]
use tokio::runtime::Runtime;

use crate::{Config, PragmaError};

#[cfg(feature = "sync")]
static INIT: Once = Once::new();
#[cfg(feature = "sync")]
static mut RUNTIME: Option<Runtime> = None;

/// HTTP client for interacting with Pragma API offchain and onchain endpoints.
///
/// # Examples
///
/// ```
/// use pragma_sdk::{Config, PragmaError, Environment, PragmaClient};
///
/// #[tokio::main]
/// async fn main() -> Result<(), PragmaError> {
///     let config = Config::new("your_api_key".to_string(), Environment::Development);
///     let client = PragmaClient::new(config)?;
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub struct PragmaClient {
    pub(crate) config: Config,
    pub(crate) http_client: reqwest::Client,
}

impl PragmaClient {
    /// Creates a new `PragmaClient` instance with the given configuration.
    pub fn new(config: Config) -> Result<Self, PragmaError> {
        let mut headers = HeaderMap::new();
        headers.insert("x-api-key", HeaderValue::from_str(&config.api_key)?);

        let http_client = Client::builder().default_headers(headers.clone()).build()?;

        Ok(PragmaClient {
            config,
            http_client,
        })
    }

    /// Gets or creates the thread-local blocking client.
    #[cfg(feature = "sync")]
    pub(crate) fn runtime(&self) -> &'static Runtime {
        unsafe {
            INIT.call_once(|| {
                RUNTIME = match Runtime::new() {
                    Ok(rt) => Some(rt),
                    Err(e) => panic!("Failed to initialize runtime: {e}"),
                };
            });
            RUNTIME.as_ref().unwrap()
        }
    }
}
