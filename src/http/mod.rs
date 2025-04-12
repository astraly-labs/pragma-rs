pub mod offchain;
pub mod onchain;

use crate::PragmaClient;

impl PragmaClient {
    /// Checks if the Pragma API is available by making a ping request.
    /// Returns true if the API responds successfully, false otherwise.
    pub async fn is_healthy(&self) -> bool {
        let url = format!("{}/node", self.config.base_url);
        match self
            .http_client
            .get(url)
            .timeout(std::time::Duration::from_secs(5))
            .send()
            .await
        {
            Ok(response) => response.status().is_success(),
            Err(_) => false,
        }
    }

    #[cfg(feature = "sync")]
    /// Synchronous version of `is_healthy`.
    /// Returns true if the API responds successfully, false otherwise.
    pub fn is_healthy_sync(&self) -> bool {
        let url = format!("{}/node", self.config.base_url);
        match self
            .http_blocking_client
            .get(url)
            .timeout(std::time::Duration::from_secs(5))
            .send()
        {
            Ok(response) => response.status().is_success(),
            Err(_) => false,
        }
    }
}
