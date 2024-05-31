use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;
use paperclip::actix::{OpenApiExt, web::Data};

mod config;
mod db;
mod handlers;
mod models;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    let config = config::Config::from_env().unwrap();
    let pool = db::create_pool(&config.database_url).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .wrap_api()
            .configure(routes::init)
            .with_json_spec_at("/api/spec/v2")
            .build()
    })
    .bind((config.server.host, config.server.port))?
    .run()
    .await
}