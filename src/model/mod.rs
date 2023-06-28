pub mod predictor;
pub mod trainer;
pub mod nn;

/// Calculates the increase between two numbers.
///
/// # Arguments
/// * `start` - The start number.
/// * `end` - The end number.
///
/// # Returns
/// The increase between the two numbers.
pub fn calculate_increase(start: f64, end: f64) -> f64 {
    (end - start) / start
}
