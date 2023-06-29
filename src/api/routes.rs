use actix_web::{get, post, web, Responder};
use log::{error, info};
use uuid::Uuid;

use crate::api::{handle_request, Request, Response};
use crate::sql::fetcher::fetch;

#[get("/")]
pub async fn index() -> impl Responder {
    "Please use the /predict endpoint to get a prediction."
}

#[post("/predict")]
pub async fn predict_post(info: web::Json<Request>) -> impl Responder {
    let info = info.into_inner();

    if info.id.is_some() {
        let error_message = "Request ID must not be set on POST requests!".to_string();
        error!("{}", error_message);

        let response = Response {
            id: None,
            status: None,
            data: None,
            error: Some(error_message),
        };

        return web::Json(response);
    }

    // Handle the request.
    let response = handle_request(&info).await;

    // Return the response as JSON.
    web::Json(response)
}

#[get("/status/{id}")]
pub async fn status(id: web::Path<Uuid>) -> impl Responder {
    info!("Status request for ID {}...", id);

    // Fetch the request by ID.
    let info = fetch(id.into_inner()).await;

    // Return the response as JSON.
    web::Json(info)
}
