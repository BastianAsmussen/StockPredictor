use log::error;
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
/// * `id` - The id of the request.
/// * `symbol` - The symbol of the stock to predict.
/// * `period` - The period to predict.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    id: Option<Uuid>,
    symbol: Option<String>,
    period: Option<Period>,

}

/// The status of the request.
///
/// # Variants
/// * `Processing` - The request is still being processed.
/// * `Finished` - The request has been finished.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Status {
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
    status: Option<Status>,
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
        status: Some(Status::Processing),
        data: None,
        error: None,
    }
}

async fn get_request(info: &Request) -> Response {
    let id = info.id.as_ref();
    if id.is_none() {
        let error_message = "Request ID must be set on GET requests!".to_string();
        error!("{}", error_message);

        return Response {
            id: None,
            status: None,
            data: None,
            error: Some(error_message),
        };
    }

    Response {
        id: info.id,
        status: Some(Status::Finished),
        data: Some("".to_string()),
        error: None,
    }
}
