use crate::util::time::as_python_datetime;
use pyo3::prelude::PyModule;
use pyo3::Python;
use time::OffsetDateTime;

pub mod predictor;
pub mod trainer;

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

/// Predicts the stock price for a given symbol and period.
///
/// # Arguments
/// * `symbol` - The symbol of the stock.
/// * `period` - The period to predict.
/// * `start` - The start date.
/// * `end` - The end date.
/// * `testing_days` - The number of days to use for testing.
/// * `future_days` - The number of days to predict into the future.
///
/// # Returns
/// A tuple containing the predictions, the increase and the accuracy.
pub async fn predict(
    symbol: &str,
    period: &str,
    start: &OffsetDateTime,
    end: &OffsetDateTime,
    testing_days: u32,
    future_days: u32,
) -> Result<(Vec<f64>, f64, f64), Box<dyn std::error::Error>> {
    // Map the start and end date to a Python datetime.
    let start = as_python_datetime(start)?;
    let end = as_python_datetime(end)?;

    // Call the Python script using Pyo3.
    Python::with_gil(|py| {
        let module_name = "predictor";
        let file_name = format!("src/model/{}.py", module_name);
        let code = std::fs::read_to_string(&file_name)?;

        let module = PyModule::from_code(py, &code, &file_name, module_name)?;
        let predict = module.getattr("predict")?;

        let args = (symbol, start, end, period, testing_days, future_days);
        let output: (Vec<f64>, f64, f64) = predict
            .call(args, None)?
            .extract()?;

        Ok(output)
    })
}
