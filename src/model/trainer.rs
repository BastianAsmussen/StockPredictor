use linfa::prelude::{Fit, Predict};
use linfa::DatasetBase;
use linfa_linear::{FittedLinearRegression, LinearRegression};
use ndarray::{ArrayBase, Ix1, Ix2, OwnedRepr};

/// Trains the model.
///
/// # Arguments
/// * `training_data` - The training data.
/// * `testing_data` - The test data.
///
/// # Returns
/// The trained model and the score of the model.
pub fn train(
    mapped_data: &DatasetBase<ArrayBase<OwnedRepr<f64>, Ix2>, ArrayBase<OwnedRepr<f64>, Ix1>>,
) -> Result<(FittedLinearRegression<f64>, f64), Box<dyn std::error::Error>> {
    // Split the mapped data into training and testing data.
    let (training_data, testing_data) = mapped_data.clone().split_with_ratio(0.9);

    // Train the model.
    let model = LinearRegression::default().fit(&training_data)?;

    // Test the model.
    let predict = model.predict(&testing_data);
    let score = r2_score(&predict, &testing_data);

    Ok((model, score))
}

/// Calculates the R2 score of the model.
///
/// # Arguments
/// * `predict` - The predictions of the model.
/// * `testing_data` - The test data.
///
/// # Returns
/// The R2 score of the model as a float (lower is better).
fn r2_score(
    predict: &ArrayBase<OwnedRepr<f64>, Ix1>,
    testing_data: &DatasetBase<ArrayBase<OwnedRepr<f64>, Ix2>, ArrayBase<OwnedRepr<f64>, Ix1>>,
) -> f64 {
    // Get the actual values of the test data.
    let y = testing_data.targets().to_vec();

    // Calculate the mean of the actual values.
    let mean = y.iter().sum::<f64>() / y.len() as f64;

    // Calculate the total sum of squares.
    let mut ss_tot = 0.0;
    for y in y.iter() {
        ss_tot += (y - mean).powi(2);
    }

    // Calculate the residual sum of squares.
    let mut ss_res = 0.0;
    for (y, y_hat) in y.iter().zip(predict.iter()) {
        ss_res += (y - y_hat).powi(2);
    }

    // Return the R2 score.
    1.0 - (ss_res / ss_tot)
}
