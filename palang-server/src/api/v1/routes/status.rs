use actix_web::{HttpResponse, Responder};

pub async fn get_status() -> impl Responder {
    HttpResponse::Ok()
}
