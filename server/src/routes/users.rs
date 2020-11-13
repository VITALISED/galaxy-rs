use actix_web::Responder;

pub async fn get() -> impl Responder {
    format!("Get User")
}

pub async fn update() -> impl Responder {
    format!("Post User")
}

pub async fn create() -> impl Responder  {
    format!("Create User")
}

pub async fn destroy() -> impl Responder {
    format!("Destroy User")
}
