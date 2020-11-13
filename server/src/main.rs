use actix_web::{web, App, HttpServer};

mod routes;
use routes::users;

use dotenv::dotenv;
use log::info;

use database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    info!("Application starting");
    database::establish_connection();

    HttpServer::new(|| {
        App::new()
            .route("/users/{id}", web::get().to(users::get))
            .route("/users/{id}", web::post().to(users::update))
            .route("/users", web::post().to(users::create))
            .route("/users/{id}", web::delete().to(users::destroy))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
