#[derive(thiserror::Error, Debug)]
pub enum PragmaError {
    /// PragmaError from the HTTP client.
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    /// PragmaError from WebSocket operations.
    #[error("WebSocket error: {0}")]
    WebSocketError(#[from] tokio_tungstenite::tungstenite::Error),

    /// PragmaError parsing JSON data.
    #[error("JSON parsing failed: {0}")]
    JsonError(#[from] serde_json::Error),

    /// PragmaError returned by the Pragma API.
    #[error("API error: {0}")]
    ApiError(String),

    #[error("Invalid header: {0}")]
    InvalidHeader(#[from] reqwest::header::InvalidHeaderValue),
}
