use pyo3::prelude::PyModule;
use pyo3::Python;

/// Predicts the stock price for a given symbol and period.
///
/// # Arguments
/// * `symbol` - The symbol of the stock.
/// * `days` - The number of days to predict into the future.
///
/// # Returns
/// A tuple containing the predictions and the RMSE of the model.
pub fn predict(symbol: &str, days: &u32) -> Result<(Vec<f64>, f64), Box<dyn std::error::Error>> {
    // Call the Python script using Pyo3.
    Python::with_gil(|py| {
        let module_name = "predictor";
        let file_name = format!("src/model/{}.py", module_name);
        let code = std::fs::read_to_string(&file_name)?;

        let module = PyModule::from_code(py, &code, &file_name, module_name)?;
        let predict = module.getattr("predict")?;

        let args = (symbol, *days);
        let output: (Vec<f64>, f64) = predict.call(args, None)?.extract()?;

        Ok(output)
    })
}
