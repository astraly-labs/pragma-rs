use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};

#[cfg(feature = "sync")]
use crate::BLOCKING_CLIENT;
use crate::{Config, PragmaError};

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

    /// Gets or creates the thread-local blocking client
    #[cfg(feature = "sync")]
    pub(crate) fn get_blocking_client(&self) -> Result<reqwest::blocking::Client, PragmaError> {
        BLOCKING_CLIENT.with(|cell| {
            let mut client_opt = cell.borrow_mut();
            if client_opt.is_none() {
                // Set up headers with the API key from config
                let mut headers = HeaderMap::new();
                headers.insert(
                    "x-api-key",
                    HeaderValue::from_str(&self.config.api_key)
                        .map_err(PragmaError::InvalidHeader)?,
                );
                // Build the client with default headers
                let client = reqwest::blocking::Client::builder()
                    .default_headers(headers)
                    .build()
                    .map_err(|_| PragmaError::BuildingClient)?;
                *client_opt = Some(client);
            }
            // Clone the client for use outside the closure
            Ok(client_opt.as_ref().unwrap().clone())
        })
    }
}
