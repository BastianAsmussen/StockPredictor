extern crate log;

use actix_web::{App, HttpServer};
use log::info;

use crate::api::routes;

mod api;
mod model;
mod util;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger.
    env_logger::init();

    // Load environment variables.
    info!("Loading environment variables...");
    let (workers, (ip, port)) = util::env::load_env();

    // Start a HTTP server.
    info!("Starting HTTP server...");
    HttpServer::new(|| App::new().service(routes::predict))
        .workers(workers)
        .bind((ip, port))?
        .run()
        .await?;

    Ok(())
}
