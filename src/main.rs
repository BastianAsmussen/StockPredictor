extern crate log;

use actix_web::{App, HttpServer};
use log::info;

use crate::api::routes;

mod data;
mod model;
mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Instantiate the logger.
    env_logger::init();

    info!("Starting up...");

    // Start a HTTP server.
    HttpServer::new(|| {
        App::new()
            .service(routes::index)
            .service(routes::predict)
    })
        .workers(4)
        .bind(("127.0.0.1", 8080))?
        .run()
        .await?;

    Ok(())
    /*
    let symbol = "AMC";
    let time = TimeUnit::Years(1);

    let pure_data = data::fetcher::fetch(symbol, &time).unwrap();
    let mapped_data = data::mapper::convert_data(&pure_data).unwrap();
    let model = model::trainer::train(&mapped_data).unwrap();

    let last_quote = pure_data.last().unwrap().1;
    let prediction = model::predictor::predict(&model, last_quote, &time).unwrap();
    let should_buy = should_buy(&model, last_quote, &time).unwrap();
    let increase = calculate_increase(pure_data.last().unwrap().1, *prediction.last().unwrap());

    println!("Symbol: {}", symbol);
    println!("Should buy? {}", should_buy);
    println!("Increase: {}%", increase);
    println!("Predicted quote: ${}", prediction.last().unwrap());
     */
}
