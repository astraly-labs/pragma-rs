use std::time::Duration;

/// Enumeration of supported time intervals for data aggregation.
///
/// This enum defines the possible time intervals over which data can be aggregated, ranging from 100 milliseconds to 1 week.
#[derive(
    Default, Debug, Clone, Copy, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum Interval {
    /// 100 milliseconds interval.
    #[serde(rename = "100ms")]
    OneHundredMillisecond,

    /// 1 second interval.
    #[serde(rename = "1s")]
    OneSecond,

    /// 5 seconds interval.
    #[serde(rename = "5s")]
    FiveSeconds,

    /// 10 seconds interval.
    #[serde(rename = "10s")]
    TenSeconds,

    /// 1 minute interval.
    #[serde(rename = "1min")]
    OneMinute,

    /// 5 minutes interval.
    #[serde(rename = "5min")]
    FiveMinutes,

    /// 15 minutes interval.
    #[serde(rename = "15min")]
    FifteenMinutes,

    /// 1 hour interval.
    #[serde(rename = "1h")]
    OneHour,

    /// 2 hours interval.
    /// This is the default variant when no specific interval is specified.
    #[serde(rename = "2h")]
    #[default]
    TwoHours,

    /// 1 day interval.
    #[serde(rename = "1d")]
    OneDay,

    /// 1 week interval.
    #[serde(rename = "1w")]
    OneWeek,
}

impl Interval {
    /// Returns the string representation of the interval as expected by the API.
    ///
    /// # Examples
    ///
    /// ```
    /// use pragma_sdk::Interval;
    ///
    /// assert_eq!(Interval::OneSecond.as_str(), "1s");
    /// assert_eq!(Interval::OneDay.as_str(), "1d");
    /// ```
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::OneHundredMillisecond => "100ms",
            Self::OneSecond => "1s",
            Self::FiveSeconds => "5s",
            Self::TenSeconds => "10s",
            Self::OneMinute => "1min",
            Self::FiveMinutes => "5min",
            Self::FifteenMinutes => "15min",
            Self::OneHour => "1h",
            Self::TwoHours => "2h",
            Self::OneDay => "1d",
            Self::OneWeek => "1w",
        }
    }

    /// Converts the interval to minutes.
    ///
    /// Returns 0 for intervals less than a minute. For larger intervals, returns the equivalent number of minutes.
    pub const fn to_minutes(&self) -> i64 {
        match self {
            Self::OneHundredMillisecond
            | Self::OneSecond
            | Self::FiveSeconds
            | Self::TenSeconds => 0,
            Self::OneMinute => 1,
            Self::FiveMinutes => 5,
            Self::FifteenMinutes => 15,
            Self::OneHour => 60,
            Self::TwoHours => 120,
            Self::OneDay => 1440,   // 24 hours * 60 minutes
            Self::OneWeek => 10080, // 7 days * 1440 minutes
        }
    }

    /// Converts the interval to seconds.
    ///
    /// Returns the exact number of seconds for intervals up to 10 seconds, and otherwise converts from minutes.
    pub const fn to_seconds(&self) -> i64 {
        if matches!(self, Self::OneHundredMillisecond) {
            return 0;
        }
        if matches!(self, Self::OneSecond) {
            return 1;
        }
        if matches!(self, Self::FiveSeconds) {
            return 5;
        }
        if matches!(self, Self::TenSeconds) {
            return 10;
        }
        self.to_minutes() * 60
    }

    /// Converts the interval to milliseconds.
    ///
    /// Returns the exact milliseconds for 100ms, and otherwise converts from seconds.
    pub const fn to_millis(&self) -> u64 {
        if matches!(self, Self::OneHundredMillisecond) {
            return 100;
        }
        (self.to_seconds() * 1000) as u64
    }
}

impl From<Interval> for Duration {
    /// Converts an `Interval` to a `Duration`.
    fn from(interval: Interval) -> Self {
        Self::from_millis(interval.to_millis())
    }
}
