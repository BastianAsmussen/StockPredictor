use actix_web::{get, Responder, web};
use serde::Deserialize;
use log::info;
use crate::data::fetcher::TimeUnit;

#[get("/")]
pub async fn index() -> impl Responder {
    "Please use the /predict endpoint to get a prediction."
}

#[get("/predict")]
pub async fn predict(info: web::Json<Request>) -> impl Responder {
    info!("Predicting... {:?}", info);

    "Predicted!"
}

#[derive(Debug, Deserialize)]
pub struct Request {
    symbol: String,
    time: TimeUnit,
}
