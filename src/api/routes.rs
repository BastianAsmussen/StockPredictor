use actix_web::{get, Responder, web};
use log::info;

use crate::api::Request;

#[get("/")]
pub async fn index() -> impl Responder {
    "Please use the /predict endpoint to get a prediction."
}

#[get("/predict")]
pub async fn predict(info: web::Json<Request>) -> impl Responder {
    // Get the request data.
    let info = info.into_inner();

    info!("Predicting for symbol {}...", info.symbol);

    // Handle the request.
    let response = crate::api::handle_request(&info).await;

    serde_json::to_string(&response).unwrap()
}
