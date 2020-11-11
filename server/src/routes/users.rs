use actix_web::Responder;

pub async fn get_users() -> impl Responder {
    format!("hello from get users")
}
