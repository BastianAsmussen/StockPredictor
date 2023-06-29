use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod routes;

/// The period to predict.
///
/// # Fields
/// * `start` - The start date.
/// * `end` - The end date.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Period {
    start: u64,
    end: u64,
}

/// The deserialized request data.
///
/// # Fields
/// * `symbol` - The symbol of the stock to predict.
/// * `period` - The period to predict.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    id: Option<Uuid>,
    symbol: Option<String>,
    period: Option<Period>,

}

/// The status of a request.
///
/// # Variants
/// * `Processing` - The request is still being processed.
/// * `Finished` - The request has been finished.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum RequestStatus {
    Processing,
    Finished,
}

/// The response data.
///
/// # Fields
/// * `id` - The id of the request.
/// * `status` - The status of the request.
/// * `data` - The data of the request.
/// * `error` - The error of the request.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    id: Option<Uuid>,
    status: Option<RequestStatus>,
    data: Option<String>,
    error: Option<String>,
}

pub async fn handle_request(info: &Request) -> Response {
    if info.id.is_none() {
        init_request(info).await
    } else {
        get_request(info).await
    }
}

async fn init_request(info: &Request) -> Response {
    Response {
        id: Some(Uuid::new_v4()),
        status: Some(RequestStatus::Processing),
        data: None,
        error: None,
    }
}

async fn get_request(info: &Request) -> Response {
    // Look up the request in the database.
    // If it exists, return the data.
    // If it doesn't exist, return an error.
    Response {
        id: Some(Uuid::new_v4()),
        status: Some(RequestStatus::Finished),
        data: None,
        error: None,
    }
}
