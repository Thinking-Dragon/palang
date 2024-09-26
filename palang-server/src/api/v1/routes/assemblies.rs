use actix_web::{HttpResponse, Responder};

pub async fn create_assembly() -> impl Responder {
    HttpResponse::Ok()
}
