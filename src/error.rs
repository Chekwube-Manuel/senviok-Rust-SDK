/// Errors returned by the Senviok client.
#[derive(Debug, thiserror::Error)]
pub enum SenviokError {
    /// Network, TLS, or HTTP-client failure.
    #[error("request failed: {0}")]
    Request(#[from] reqwest::Error),

    /// Response body was not valid JSON for the expected type.
    #[error("failed to decode response: {0}")]
    Decode(#[from] serde_json::Error),

    /// The API returned a non-success status code.
    #[error("api error {status}: {message}")]
    Api {
        /// HTTP status from the API.
        status: reqwest::StatusCode,
        /// Parsed error message when the body includes one.
        message: String,
        /// Raw response body.
        body: String,
    },
}
