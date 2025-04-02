/// Environment options for the Pragma API.
#[derive(Debug, Clone, Copy)]
pub enum Environment {
    Development,
    Production,
}

/// Configuration for the Pragma SDK.
///
/// Holds the API key and environment-specific URLs for HTTP and WebSocket connections.
///
/// # Examples
///
/// ```
/// use pragma_sdk::{Config, Environment};
///
/// let config = Config::new("your_api_key".to_string(), Environment::Development);
/// ```
pub struct Config {
    pub api_key: String,
    pub base_url: String,
    pub ws_url: String,
}

impl Config {
    /// Creates a new `Config` instance with the given API key and environment.
    pub fn new(api_key: String, environment: Environment) -> Self {
        let (base_url, ws_url) = match environment {
            Environment::Development => (
                "https://api.devnet.pragma.build/node/v1".to_string(),
                "wss://ws.devnet.pragma.build/node/v1/data/subscribe".to_string(),
            ),
            Environment::Production => todo!("Not sure yet."),
        };
        Config {
            api_key,
            base_url,
            ws_url,
        }
    }
}
