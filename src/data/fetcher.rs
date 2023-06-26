use regex::Regex;
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};
use yahoo_finance_api as yahoo;

/// Fetches the data from Yahoo Finance for the given exchange and symbol.
///
/// # Arguments
/// * `symbol` - The symbol to fetch the data for.
/// * `time` - The time unit to fetch the data for.
///
/// # Returns
/// A vector containing tuples of the open and adjusted close prices, or an error.
pub async fn fetch(symbol: &str, time: &TimeUnit) -> Result<Vec<(f64, f64)>, Box<dyn std::error::Error>> {
    // Make sure the ticker symbol is valid.
    validate_symbol(symbol)?;

    let provider = yahoo::YahooConnector::new();

    // Find out when the symbol was first listed.
    let (start, end) = get_time(time.as_seconds());

    let response = provider.get_quote_history(symbol, start, end).await?;
    let quotes = response.quotes()?;

    // Map the quotes to a vector of tuples containing the open and adjusted close prices.
    let data = quotes
        .iter()
        .map(|quote| (quote.open, quote.adjclose))
        .collect::<Vec<(f64, f64)>>();

    Ok(data)
}

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
#[serde(tag = "time", content = "value")]
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

/// Gets the current time and the time `seconds` ago.
///
/// # Arguments
/// * `seconds` - The number of seconds ago to get the time for.
///
/// # Returns
/// A tuple containing the start and end times.
fn get_time(seconds: u64) -> (OffsetDateTime, OffsetDateTime) {
    let now = OffsetDateTime::now_utc();
    let start = now - Duration::seconds(seconds as i64);

    (start, now)
}

/// Validates the given symbol.
///
/// # Arguments
/// * `symbol` - The symbol to validate.
///
/// # Returns
/// `Ok(())` if the symbol is valid, otherwise an error.
fn validate_symbol(symbol: &str) -> Result<(), Box<dyn std::error::Error>> {
    match Regex::new(r"^[A-Z]{1,5}$") {
        Ok(regex) => {
            if !regex.is_match(symbol) {
                return Err("Invalid ticker symbol!".into());
            }

            Ok(())
        }
        Err(_) => Err("Invalid ticker symbol!".into()),
    }
}
