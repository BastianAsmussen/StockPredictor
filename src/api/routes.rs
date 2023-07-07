use actix_web::{get, web, Responder};

use crate::api::{handle_request, Request};

#[get("/predict")]
pub async fn predict(request: web::Json<Request>) -> impl Responder {
    // Get the request data.
    let request = request.into_inner();

    // Handle the request.
    let response = handle_request(&request).await;

    // Return the response as JSON.
    web::Json(response)
}
