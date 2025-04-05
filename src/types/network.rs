#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum Network {
    #[serde(rename = "sepolia")]
    Sepolia,
    #[serde(rename = "mainnet")]
    Mainnet,
}

impl Network {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Sepolia => "sepolia",
            Self::Mainnet => "mainnet",
        }
    }
}

impl Default for Network {
    fn default() -> Self {
        Self::Mainnet
    }
}
