use actix_web::Responder;

pub async fn get() -> impl Responder {
    format!("Get User")
}

pub async fn update() {
    format!("Post User")
}

pub async fn create() {
    format!("Create User")
}

pub async fn destroy() {
    format!("Destroy User")
}
