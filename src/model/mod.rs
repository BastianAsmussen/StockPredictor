use linfa_linear::FittedLinearRegression;

use crate::data::fetcher::TimeUnit;

pub mod predictor;
pub mod trainer;

/// Calculates the percentage increase between two numbers.
pub fn calculate_increase(start: f64, end: f64) -> f64 {
    (end - start) / start * 100.0
}

/// Determines whether you should buy or sell.
///
/// # Arguments
/// * `model` - The model to use for the prediction.
/// * `last_quote` - The last quote.
/// * `time` - The time to predict in the future.
///
/// # Returns
/// True if you should buy, false if you should sell.
pub fn should_buy(
    model: &FittedLinearRegression<f64>,
    last_quote: f64,
    time: &TimeUnit,
) -> Result<bool, Box<dyn std::error::Error>> {
    let prediction = predictor::predict(model, last_quote, time)?;
    let last_prediction = if let Some(last_prediction) = prediction.last() {
        last_prediction
    } else {
        return Err("No prediction!".into());
    };

    let increase = calculate_increase(last_quote, *last_prediction);

    Ok(increase > 0.0)
}
