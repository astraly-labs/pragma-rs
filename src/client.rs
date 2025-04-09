use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};

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
    #[cfg(feature = "sync")]
    pub(crate) http_blocking_client: reqwest::blocking::Client,
}

impl PragmaClient {
    /// Creates a new `PragmaClient` instance with the given configuration.
    pub fn new(config: Config) -> Result<Self, PragmaError> {
        let mut headers = HeaderMap::new();
        headers.insert("x-api-key", HeaderValue::from_str(&config.api_key)?);

        let http_client = Client::builder().default_headers(headers.clone()).build()?;
        #[cfg(feature = "sync")]
        let http_blocking_client = reqwest::blocking::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(PragmaClient {
            config,
            http_client,
            #[cfg(feature = "sync")]
            http_blocking_client,
        })
    }
}
