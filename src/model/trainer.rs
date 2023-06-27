use linfa::prelude::Fit;
use linfa::DatasetBase;
use linfa_linear::{FittedLinearRegression, LinearRegression};
use ndarray::{ArrayBase, Ix1, Ix2, OwnedRepr};

/// Trains the model.
///
/// # Arguments
/// * `mapped_data` - The data to train the model with.
///
/// # Returns
/// The trained model.
pub fn train(
    mapped_data: &DatasetBase<ArrayBase<OwnedRepr<f64>, Ix2>, ArrayBase<OwnedRepr<f64>, Ix1>>,
) -> Result<FittedLinearRegression<f64>, Box<dyn std::error::Error>> {
    let model = LinearRegression::default().fit(mapped_data)?;

    Ok(model)
}
