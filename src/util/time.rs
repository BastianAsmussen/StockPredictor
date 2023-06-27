use std::fmt::Display;

use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};

/// The smallest amount of time that can be used.
pub const MINIMUM_TIME: u64 = 1;
/// The largest amount of time that can be used.
pub const MAXIMUM_TIME: u64 = i64::MAX as u64;

/// The time unit to fetch the data for.
///
/// # Variants
/// * `Seconds` - The number of seconds ago to fetch the data for.
/// * `Minutes` - The number of minutes ago to fetch the data for.
/// * `Hours` - The number of hours ago to fetch the data for.
/// * `Days` - The number of days ago to fetch the data for.
/// * `Weeks` - The number of weeks ago to fetch the data for.
/// * `Months` - The number of months ago to fetch the data for.
/// * `Years` - The number of years ago to fetch the data for.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "unit", content = "value")]
#[serde(rename_all = "camelCase")]
pub enum TimeUnit {
    Seconds(u64),
    Minutes(u64),
    Hours(u64),
    Days(u64),
    Weeks(u64),
    Months(u64),
    Years(u64),
}

impl TimeUnit {
    /// Gets the number of the time unit.
    ///
    /// # Returns
    /// The number of the time unit.
    pub fn get_number(&self) -> u64 {
        match self {
            TimeUnit::Seconds(seconds) => *seconds,
            TimeUnit::Minutes(minutes) => *minutes,
            TimeUnit::Hours(hours) => *hours,
            TimeUnit::Days(days) => *days,
            TimeUnit::Weeks(weeks) => *weeks,
            TimeUnit::Months(months) => *months,
            TimeUnit::Years(years) => *years,
        }
    }
    /// Converts the time unit to seconds.
    ///
    /// # Returns
    /// The number of seconds in the time unit.
    pub fn as_seconds(&self) -> u64 {
        match self {
            TimeUnit::Seconds(seconds) => *seconds,
            TimeUnit::Minutes(minutes) => *minutes * 60,
            TimeUnit::Hours(hours) => *hours * 60 * 60,
            TimeUnit::Days(days) => *days * 60 * 60 * 24,
            TimeUnit::Weeks(weeks) => *weeks * 60 * 60 * 24 * 7,
            TimeUnit::Months(months) => *months * 60 * 60 * 24 * 30,
            TimeUnit::Years(years) => *years * 60 * 60 * 24 * 365,
        }
    }
}

impl Display for TimeUnit {
    /// Formats the time unit.
    ///
    /// # Arguments
    /// * `f` - The formatter.
    ///
    /// # Returns
    /// A result containing the formatted time unit.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeUnit::Seconds(seconds) => write!(f, "{} seconds", seconds),
            TimeUnit::Minutes(minutes) => write!(f, "{} minutes", minutes),
            TimeUnit::Hours(hours) => write!(f, "{} hours", hours),
            TimeUnit::Days(days) => write!(f, "{} days", days),
            TimeUnit::Weeks(weeks) => write!(f, "{} weeks", weeks),
            TimeUnit::Months(months) => write!(f, "{} months", months),
            TimeUnit::Years(years) => write!(f, "{} years", years),
        }
    }
}

/// Gets the current time and the time `seconds` ago.
///
/// # Arguments
/// * `seconds` - The number of seconds ago to get the time for.
///
/// # Returns
/// A tuple containing the start and end times.
pub fn get_time(seconds: u64) -> Result<(OffsetDateTime, OffsetDateTime), Box<dyn std::error::Error>> {
    // If the seconds exceeds the minimum or maximum time, return an error.
    if !(MINIMUM_TIME..=MAXIMUM_TIME).contains(&seconds) {
        return Err(format!("The number of seconds must be between {:.} and {:.}!", MINIMUM_TIME, MAXIMUM_TIME).into());
    }

    // Make sure the time is not in the future.
    let now = OffsetDateTime::now_utc();
    if seconds > now.to_offset(now.offset()).unix_timestamp() as u64 {
        return Err("The time cannot be in the future!".into());
    }

    // Get the start and end times.
    let start = now - Duration::seconds(seconds as i64);

    Ok((start, now))
}
