use crate::data::fetcher::fetch;
use log::error;
use serde::{Deserialize, Serialize};

use crate::data::mapper::convert_data;
use crate::model::calculate_increase;
use crate::model::predictor::predict;
use crate::model::trainer::train;
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
/// * `error_message` - The error message.
/// * `current_adjusted_close` - The current adjusted close price.
/// * `model_score` - The model score (lower is better).
/// * `predictions` - The predictions.
/// * `increase` - The increase.
/// * `should_buy` - Whether or not the stock should be bought.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    request: Request,
    error_message: Option<String>,
    current_adjusted_close: Option<f64>,
    model_r2_score: Option<f64>,
    predictions: Option<Vec<f64>>,
    increase: Option<f64>,
    should_buy: Option<bool>,
}

pub async fn handle_request(info: &Request) -> Response {
    // Fetch the data.
    let pure_data = fetch(&info.symbol, &info.dataset_size).await;
    if pure_data.is_err() {
        let error_message = format!(
            "Failed to fetch data for symbol {}! {}",
            info.symbol,
            pure_data.err().unwrap()
        );
        error!("{}", error_message);

        return Response {
            request: info.clone(),
            error_message: Some(error_message),
            current_adjusted_close: None,
            model_r2_score: None,
            predictions: None,
            increase: None,
            should_buy: None,
        };
    }
    let pure_data = pure_data.unwrap();

    // Get the last quote.
    let (open, adj_close) = *pure_data.last().unwrap();

    // Convert the data.
    let mapped_data = convert_data(&pure_data);
    if mapped_data.is_err() {
        let error_message = format!(
            "Failed to convert data for symbol {}! {}",
            info.symbol,
            mapped_data.err().unwrap()
        );
        error!("{}", error_message);

        return Response {
            request: info.clone(),
            error_message: Some(error_message),
            current_adjusted_close: Some(adj_close),
            model_r2_score: None,
            predictions: None,
            increase: None,
            should_buy: None,
        };
    }
    let mapped_data = mapped_data.unwrap();

    // Train the model.
    let model = train(&mapped_data);
    if model.is_err() {
        let error_message = format!(
            "Failed to train model for symbol {}! {}",
            info.symbol,
            model.err().unwrap()
        );
        error!("{}", error_message);

        return Response {
            request: info.clone(),
            error_message: Some(error_message),
            current_adjusted_close: Some(adj_close),
            model_r2_score: None,
            predictions: None,
            increase: None,
            should_buy: None,
        };
    }
    let (model, r2_score) = model.unwrap();

    let predictions = predict(&model, open, &info.time);
    if predictions.is_err() {
        let error_message = format!(
            "Failed to predict data for symbol {}! {}",
            info.symbol,
            predictions.err().unwrap()
        );
        error!("{}", error_message);

        return Response {
            request: info.clone(),
            error_message: Some(error_message),
            current_adjusted_close: Some(adj_close),
            model_r2_score: Some(r2_score),
            predictions: None,
            increase: None,
            should_buy: None,
        };
    }
    let predictions = predictions.unwrap();

    // Calculate the increase.
    let start = adj_close;
    let end = *predictions.last().unwrap();
    let increase = calculate_increase(start, end);

    // Calculate whether or not the stock should be bought.
    let should_buy = increase > 0.0;

    // Return the response.
    Response {
        request: info.clone(),
        error_message: None,
        current_adjusted_close: Some(adj_close),
        model_r2_score: Some(r2_score),
        predictions: Some(predictions),
        increase: Some(increase),
        should_buy: Some(should_buy),
    }
}
