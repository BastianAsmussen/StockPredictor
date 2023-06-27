pub mod predictor;
pub mod trainer;

/// Calculates the percentage increase between two numbers.
///
/// # Arguments
/// * `start` - The start number.
/// * `end` - The end number.
///
/// # Returns
/// The percentage increase between the two numbers.
pub fn calculate_increase(start: f64, end: f64) -> f64 {
    (end - start) / start * 100.0
}
