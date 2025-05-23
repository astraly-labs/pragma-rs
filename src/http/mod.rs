pub(crate) mod offchain;
pub(crate) mod onchain;

use crate::PragmaClient;

impl PragmaClient {
    /// Checks if the Pragma API is available by making a ping request.
    /// Returns true if the API responds successfully, false otherwise.
    pub async fn is_healthy(&self) -> bool {
        let url = format!("{}/node", self.config.base_url);

        (self
            .http_client
            .get(url)
            .timeout(std::time::Duration::from_secs(2))
            .send()
            .await)
            .map_or(false, |response| response.status().is_success())
    }

    #[cfg(feature = "sync")]
    pub fn is_healthy_sync(&self) -> bool {
        let runtime = Self::runtime();
        runtime.block_on(self.is_healthy())
    }
}
