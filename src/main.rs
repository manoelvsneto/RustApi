use actix_web::{web, App, HttpServer};
use sqlx::MssqlPool;
use dotenv::dotenv;

mod db;
mod handlers;
mod models;
mod swagger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let pool = db::establish_connection().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(web::resource("/pessoas")
                .route(web::get().to(handlers::get_pessoas))
                .route(web::post().to(handlers::create_pessoa)))
            .service(web::resource("/pessoas/{id}")
                .route(web::get().to(handlers::get_pessoa_by_id))
                .route(web::put().to(handlers::update_pessoa))
                .route(web::delete().to(handlers::delete_pessoa)))
            .service(swagger::swagger())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
