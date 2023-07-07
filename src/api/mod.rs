use log::error;
use serde::{Deserialize, Serialize};

use crate::model::predict;

pub mod routes;

/// The deserialized request data.
///
/// # Fields
/// * `symbol` - The symbol of the stock to predict.
/// * `days` - The number of days to predict into the future.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    symbol: Option<String>,
    days: Option<u32>,
}

/// The response data.
///
/// # Fields
/// * `request` - The request data.
/// * `error` - The error message if an error occurred.
/// * `predictions` - The predictions if the request was successful.
/// * `model_rmse` - The RMSE of the model if the request was successful.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    request: Request,
    error: Option<String>,
    predictions: Option<Vec<f64>>,
    model_rmse: Option<f64>,
}

pub async fn handle_request(request: &Request) -> Response {
    let symbol = if let Some(symbol) = &request.symbol {
        symbol
    } else {
        let error_message = "Stock ticker symbol must be set!".to_string();
        error!("{}", error_message);

        return Response {
            request: request.clone(),
            error: Some(error_message),
            predictions: None,
            model_rmse: None,
        };
    };

    let days = if let Some(days) = &request.days {
        days
    } else {
        let error_message = "Future days must be set!".to_string();
        error!("{}", error_message);

        return Response {
            request: request.clone(),
            error: Some(error_message),
            predictions: None,
            model_rmse: None,
        };
    };

    // Process the request.
    match predict(symbol, days) {
        Ok(data) => Response {
            request: request.clone(),
            error: None,
            predictions: Some(data.0),
            model_rmse: Some(data.1),
        },
        Err(error) => {
            let error_message = format!("An error occurred: {}", error);
            error!("{}", error_message);

            Response {
                request: request.clone(),
                error: Some(error_message),
                predictions: None,
                model_rmse: None,
            }
        }
    }
}
