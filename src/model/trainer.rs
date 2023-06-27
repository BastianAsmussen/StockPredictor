use linfa::dataset::Records;
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
    // Split the data into training and testing data.
    let (training_data, testing_data) = mapped_data.clone().split_with_ratio(0.8);

    // Train the model.
    let model = LinearRegression::default().fit(&training_data)?;

    // Predict the test data.
    let predict = model.predict(testing_data.records());
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
    let mut sum = 0.0;
    for (prediction, actual) in predict.iter().zip(testing_data.records()) {
        sum += (prediction - actual).powi(2);
    }
    1.0 - sum / testing_data.nsamples() as f64 / testing_data.records().nfeatures() as f64
}
