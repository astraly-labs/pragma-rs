/// Enumeration of possible aggregation modes for price data.
///
/// This enum defines the supported methods for aggregating price data, such as median or time-weighted average price (TWAP).
#[derive(Default, Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum AggregationMode {
    /// Represents the median aggregation mode.
    #[serde(rename = "median")]
    Median,

    /// Represents the time-weighted average price (TWAP) aggregation mode.
    /// This is the default variant when no specific mode is specified.
    #[serde(rename = "twap")]
    #[default]
    Twap,
}

impl AggregationMode {
    /// Returns the string representation of the aggregation mode as expected by the API.
    ///
    /// # Examples
    ///
    /// ```
    /// use pragma_sdk::AggregationMode;
    ///
    /// assert_eq!(AggregationMode::Median.as_str(), "median");
    /// assert_eq!(AggregationMode::Twap.as_str(), "twap");
    /// ```
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Median => "median",
            Self::Twap => "twap",
        }
    }
}
