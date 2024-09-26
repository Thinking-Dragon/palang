use actix_web::{HttpResponse, Responder};

pub async fn get_projects() -> impl Responder {
    HttpResponse::Ok()
}

pub async fn create_project() -> impl Responder {
    HttpResponse::Ok()
}
