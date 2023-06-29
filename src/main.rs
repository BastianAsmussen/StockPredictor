extern crate log;

use actix_web::{App, HttpServer};
use log::{info, warn};

use crate::api::routes;

mod api;
mod model;
mod util;

/// The default amount of threads to use for the HTTP server.
const DEFAULT_WORKERS: usize = 4;

/// The default IP to bind to.
const DEFAULT_IP: &str = "127.0.0.1";
/// The default port to bind to.
const DEFAULT_PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger.
    env_logger::init();

    // Load environment variables.
    info!("Loading environment variables...");
    let (workers, (ip, port)) = load_env();

    // Start a HTTP server.
    info!("Starting HTTP server...");
    HttpServer::new(|| App::new().service(routes::index).service(routes::predict_post))
        .workers(workers)
        .bind((ip, port))?
        .run()
        .await?;

    Ok(())
}

/// Loads the environment variables.
///
/// # Returns
/// A tuple containing the amount of workers, the IP and the port.
fn load_env() -> (usize, (String, u16)) {
    dotenv::dotenv().ok();

    let workers = dotenv::var("WORKERS")
        .unwrap_or_else(|_| {
            warn!("WORKERS not set, using {}...", DEFAULT_WORKERS);

            DEFAULT_WORKERS.to_string()
        })
        .parse()
        .expect("WORKERS must be a number!");

    let ip = dotenv::var("IP").unwrap_or_else(|_| {
        warn!("IP not set, using {}...", DEFAULT_IP);

        DEFAULT_IP.to_string()
    });
    let port = dotenv::var("PORT")
        .unwrap_or_else(|_| {
            warn!("PORT not set, using {}...", DEFAULT_PORT);

            DEFAULT_PORT.to_string()
        })
        .parse()
        .expect("PORT must be a number!");

    (workers, (ip, port))
}
