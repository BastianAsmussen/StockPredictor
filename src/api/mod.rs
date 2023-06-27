use log::error;

use crate::model::calculate_increase;
use serde::{Deserialize, Serialize};
use crate::util::time::TimeUnit;

pub mod routes;

/// The deserialized request data.
///
/// # Fields
/// * `symbol` - The symbol of the stock to predict.
/// * `time` - The time unit to use for the prediction.
/// * `dataset_size` - The size of the dataset to use for the prediction.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    symbol: String,
    time: TimeUnit,
    dataset_size: TimeUnit,
}

/// The serialized response data.
///
/// # Fields
/// * `request` - The request data.
/// * `predictions` - The predictions.
/// * `increase` - The increase.
/// * `should_buy` - Whether or not the stock should be bought.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    request: Request,
    error_message: Option<String>,
    predictions: Option<Vec<f64>>,
    increase: Option<f64>,
    should_buy: Option<bool>,
}

pub async fn handle_request(info: &Request) -> Response {
    // Fetch the data.
    let pure_data = crate::data::fetcher::fetch(&info.symbol, &info.dataset_size).await;
    if pure_data.is_err() {
        let error_message = format!("Failed to fetch data for symbol {}! {}",
            info.symbol,
            pure_data.err().unwrap()
        );
        error!("{}", error_message);

        return Response {
            request: info.clone(),
            error_message: Some(error_message),
            predictions: None,
            increase: None,
            should_buy: None,
        };
    }
    let pure_data = pure_data.unwrap();

    // Convert the data.
    let mapped_data = crate::data::mapper::convert_data(&pure_data);
    if mapped_data.is_err() {
        let error_message = format!("Failed to convert data for symbol {}! {}",
            info.symbol,
            mapped_data.err().unwrap()
        );
        error!("{}", error_message);

        return Response {
            request: info.clone(),
            error_message: Some(error_message),
            predictions: None,
            increase: None,
            should_buy: None,
        };
    }
    let mapped_data = mapped_data.unwrap();

    // Train the model.
    let model = crate::model::trainer::train(&mapped_data);
    if model.is_err() {
        let error_message = format!("Failed to train model for symbol {}! {}",
            info.symbol,
            model.err().unwrap()
        );
        error!("{}", error_message);

        return Response {
            request: info.clone(),
            error_message: Some(error_message),
            predictions: None,
            increase: None,
            should_buy: None,
        };
    }
    let model = model.unwrap();

    // Predict the data.
    let (.., close) = pure_data.last().unwrap();

    let predictions = crate::model::predictor::predict(&model, *close, &info.time);
    if predictions.is_err() {
        let error_message = format!("Failed to predict data for symbol {}! {}",
            info.symbol,
            predictions.err().unwrap()
        );
        error!("{}", error_message);

        return Response {
            request: info.clone(),
            error_message: Some(error_message),
            predictions: None,
            increase: None,
            should_buy: None,
        };
    }
    let predictions = predictions.unwrap();

    // Calculate the increase.
    let start = *close;
    let end = *predictions.last().unwrap();
    let increase = calculate_increase(start, end);

    // Calculate whether or not the stock should be bought.
    let should_buy = increase > 0.0;

    // Return the response.
    Response {
        request: info.clone(),
        error_message: None,
        predictions: Some(predictions),
        increase: Some(increase),
        should_buy: Some(should_buy),
    }
}
