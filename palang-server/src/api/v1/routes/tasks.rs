use actix_web::{HttpResponse, Responder};

pub async fn run_task() -> impl Responder {
    HttpResponse::Ok()
}
