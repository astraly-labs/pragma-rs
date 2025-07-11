/// Environment options for the Pragma API.
#[derive(Debug, Clone)]
pub enum Environment {
    Local {
        http_base_url: String,
        ws_base_url: String,
    },
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
/// use pragma_rs::{Config, Environment};
///
/// let config = Config::new("your_api_key".to_string(), Environment::Development);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub(crate) api_key: String,
    pub(crate) base_url: String,
    #[allow(unused)]
    pub(crate) ws_url: String,
}

impl Config {
    /// Creates a new `Config` instance with the given API key and environment.
    pub fn new(api_key: String, environment: Environment) -> Self {
        let (base_url, ws_url) = match environment {
            Environment::Local {
                http_base_url,
                ws_base_url,
            } => (http_base_url, ws_base_url),
            Environment::Development => (
                "https://api.devnet.pragma.build".to_string(),
                "wss://api.devnet.pragma.build".to_string(),
            ),
            Environment::Production => (
                "https://api.production.pragma.build".to_string(),
                "wss://api.production.pragma.build".to_string(),
            ),
        };
        Self {
            api_key,
            base_url,
            ws_url,
        }
    }
}
