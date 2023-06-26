use crate::data::fetcher::TimeUnit;
use linfa::prelude::Predict;
use linfa_linear::FittedLinearRegression;
use ndarray::Array;

/// Predicts the next quote based on the last quote.
///
/// # Arguments
/// * `model` - The model to use for the prediction.
/// * `last_quote` - The last quote.
/// * `time` - The time to predict in the future.
///
/// # Returns
/// A vector containing the predicted quotes.
pub fn predict(
    model: &FittedLinearRegression<f64>,
    mut last_quote: f64,
    time: &TimeUnit,
) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
    let array = Array::from_elem((1, 1), last_quote);

    let mut predictions = Vec::new();
    for _ in 0..time.get_number() {
        let prediction = model.predict(&array);
        last_quote = *prediction.first().unwrap();

        predictions.push(last_quote);
    }

    Ok(predictions)
}
