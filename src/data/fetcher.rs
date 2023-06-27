use regex::Regex;
use yahoo_finance_api as yahoo;

use crate::util::time::{get_time, TimeUnit};

/// Fetches the data from Yahoo Finance for the given exchange and symbol.
///
/// # Arguments
/// * `symbol` - The symbol to fetch the data for.
/// * `dataset_size` - The size of the dataset to fetch.
///
/// # Returns
/// A vector containing tuples of the open and adjusted close prices, or an error.
pub async fn fetch(
    symbol: &str,
    dataset_size: &TimeUnit,
) -> Result<Vec<(f64, f64)>, Box<dyn std::error::Error>> {
    // Make sure the ticker symbol is valid.
    validate_symbol(symbol)?;

    let provider = yahoo::YahooConnector::new();

    // Find out when the symbol was first listed.
    let (start, end) = get_time(dataset_size.as_seconds())?;

    let response = provider.get_quote_history(symbol, start, end).await?;
    let quotes = response.quotes()?;

    // Map the quotes to a vector of tuples containing the open and adjusted close prices.
    let data = quotes
        .iter()
        .map(|quote| (quote.close, quote.adjclose))
        .collect::<Vec<(f64, f64)>>();

    Ok(data)
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
