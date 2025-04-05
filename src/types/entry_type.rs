/// Enumeration of possible entry types for data retrieval.
///
/// This enum defines the types of entries that can be retrieved, such as spot or perpetual (perp) prices.
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum EntryType {
    /// Represents spot price data.
    #[serde(rename = "spot")]
    Spot,
    /// Represents perpetual (perp) price data.
    #[serde(rename = "perp")]
    Perp,
}

impl EntryType {
    /// Returns the string representation of the entry type as expected by the API.
    ///
    /// # Examples
    ///
    /// ```
    /// use your_crate_name::EntryType; // Replace with your crate name
    ///
    /// assert_eq!(EntryType::Spot.as_str(), "spot");
    /// assert_eq!(EntryType::Perp.as_str(), "perp");
    /// ```
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Spot => "spot",
            Self::Perp => "perp",
        }
    }
}
