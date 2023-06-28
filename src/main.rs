extern crate log;

use actix_web::{App, HttpServer};
use log::info;

use crate::api::routes;

mod api;
mod data;
mod model;
mod util;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger.
    env_logger::init();

    info!("Starting up...");

    // Start a HTTP server.
    HttpServer::new(|| App::new().service(routes::index).service(routes::predict))
        .workers(4)
        .bind(("127.0.0.1", 8080))?
        .run()
        .await?;

    Ok(())
}
