use actix_web::{get, web, Responder};
use log::info;

use crate::api::{handle_request, Request};

#[get("/")]
pub async fn index() -> impl Responder {
    "Please use the /predict endpoint to get a prediction."
}

#[get("/predict")]
pub async fn predict(info: web::Json<Request>) -> impl Responder {
    // Get the request data.
    let info = info.into_inner();

    info!(
        "Received request for symbol {} in {:.} with dataset size of {:.}.",
        info.symbol, info.time, info.dataset_size
    );

    // Handle the request.
    let response = handle_request(&info).await;

    // Return the response as JSON.
    web::Json(response)
}
