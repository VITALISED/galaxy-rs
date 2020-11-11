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

    HttpServer::new(move || App::new().route("/users", web::get().to(users::get_users)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
