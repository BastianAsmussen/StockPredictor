use linfa::{Dataset, DatasetBase};
use ndarray::{Array, ArrayBase, Ix1, Ix2, OwnedRepr};

/// Converts the fetched data into an `ndarray` dataset.
///
/// # Arguments
/// * `data` - The data to convert.
///
/// # Returns
/// The dataset.
pub fn convert_data(
    data: &[(f64, f64)],
) -> Result<
    DatasetBase<ArrayBase<OwnedRepr<f64>, Ix2>, ArrayBase<OwnedRepr<f64>, Ix1>>,
    Box<dyn std::error::Error>,
> {
    let mut x = Vec::new();
    let mut y = Vec::new();

    // Iterate over the data and split it into the open and adjusted close prices.
    for (open, close) in data.iter() {
        x.push(*close);
        y.push(*open);
    }

    // Convert the vectors into arrays.
    let x = Array::from(x.clone()).into_shape((x.len(), 1))?;
    let y = Array::from(y);

    let dataset = Dataset::new(x, y);

    Ok(dataset)
}
