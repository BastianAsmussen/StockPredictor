use crate::data::fetcher::TimeUnit;

use serde::{Deserialize, Serialize};

pub mod routes;

/// The deserialized request data.
///
/// # Fields
/// * `symbol` - The symbol of the stock to predict.
/// * `time` - The time unit to use.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    symbol: String,
    time: TimeUnit,
}

/// The serialized response data.
///
/// # Fields
/// * `request` - The request data.
/// * `predictions` - The predictions.
/// * `increase` - The increase.
/// * `should_buy` - Whether or not the stock should be bought.
#[derive(Debug, Serialize)]
pub struct Response {
    request: Request,
    predictions: Option<Vec<f64>>,
    increase: Option<f64>,
    should_buy: Option<bool>,
}

pub async fn handle_request(info: &Request) -> Response {
    // Fetch the data.
    let pure_data = crate::data::fetcher::fetch(&info.symbol, &info.time).await;
    if pure_data.is_err() {
        return Response {
            request: info.clone(),
            predictions: None,
            increase: None,
            should_buy: None,
        };
    }
    let pure_data = pure_data.unwrap();

    // Convert the data.
    let mapped_data = crate::data::mapper::convert_data(&pure_data);
    if mapped_data.is_err() {
        return Response {
            request: info.clone(),
            predictions: None,
            increase: None,
            should_buy: None,
        };
    }
    let mapped_data = mapped_data.unwrap();

    // Train the model.
    let model = crate::model::trainer::train(&mapped_data);
    if model.is_err() {
        return Response {
            request: info.clone(),
            predictions: None,
            increase: None,
            should_buy: None,
        };
    }
    let model = model.unwrap();

    // Predict the data.
    let last_quote = pure_data.last().unwrap().1;

    let prediction = crate::model::predictor::predict(&model, last_quote, &info.time);
    if prediction.is_err() {
        return Response {
            request: info.clone(),
            predictions: None,
            increase: None,
            should_buy: None,
        };
    }
    let prediction = prediction.unwrap();

    // Calculate the increase.
    let increase = crate::model::calculate_increase(last_quote, *prediction.last().unwrap());

    // Calculate whether or not the stock should be bought.
    let should_buy = crate::model::should_buy(&model, last_quote, &info.time);
    if should_buy.is_err() {
        return Response {
            request: info.clone(),
            predictions: None,
            increase: None,
            should_buy: None,
        };
    }
    let should_buy = should_buy.unwrap();

    // Return the response.
    Response {
        request: info.clone(),
        predictions: Some(prediction),
        increase: Some(increase),
        should_buy: Some(should_buy),
    }
}
