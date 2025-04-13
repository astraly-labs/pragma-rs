#[derive(thiserror::Error, Debug)]
pub enum PragmaError {
    /// Unauthorized access (HTTP 401).
    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    /// PragmaError from the HTTP client.
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    /// PragmaError from WebSocket operations.
    #[error("WebSocket error: {0}")]
    WebSocketError(#[from] tokio_tungstenite::tungstenite::Error),

    #[error("Could not build Pragma client")]
    BuildingClient,

    /// PragmaError parsing JSON data.
    #[error("JSON parsing failed: {0}")]
    JsonError(#[from] serde_json::Error),

    /// PragmaError returned by the Pragma API.
    #[error("API error: {0}")]
    ApiError(String),

    #[error("Invalid header: {0}")]
    InvalidHeader(#[from] reqwest::header::InvalidHeaderValue),

    #[error("Could not serialize: {0}")]
    SerializationError(String),

    #[error("Channel error: {0}")]
    ChannelError(String),
}
