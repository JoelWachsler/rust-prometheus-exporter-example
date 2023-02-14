mod config;
mod logger;
mod metrics;

use crate::config::Config;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::path::Path;

#[get("/")]
async fn index() -> impl Responder {
    "ok"
}

// inspired by https://github.com/tikv/rust-prometheus/blob/master/examples/example_hyper.rs
#[get("/metrics")]
async fn metrics_endpoint(data: web::Data<Config>) -> impl Responder {
    match metrics::update_and_get_metrics(&data).await {
        Ok(res) => HttpResponse::Ok().body(res),
        Err(err) => HttpResponse::InternalServerError().body(format!("{err:?}")),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    logger::init().unwrap();
    let config = Config::from_path(Path::new("./config.yaml"))
        .await
        .unwrap_or_else(|e| panic!("Failed to parse config.yaml, {:?}", e));

    logger::info!("Parsed the following config.yaml\n{:#?}", config);
    let data = web::Data::new(config);

    let server = HttpServer::new(move || {
        App::new()
            .service(index)
            .app_data(data.clone())
            .service(metrics_endpoint)
    })
    .workers(2)
    .bind(("0.0.0.0", 8080))?;

    let addr = server.addrs();
    logger::info!("Listening to {addr:?}");

    server.run().await
}
