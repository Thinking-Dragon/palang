use actix_web::{HttpResponse, Responder};

pub async fn get_profiles() -> impl Responder {
    HttpResponse::Ok()
}

pub async fn create_profile() -> impl Responder {
    HttpResponse::Ok()
}

pub async fn create_profile_alias() -> impl Responder {
    HttpResponse::Ok()
}
