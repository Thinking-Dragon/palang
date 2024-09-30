use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

use crate::api::v1::{models::profile::Profile, services::{profile::ProfileService, storage::Storable}};

pub async fn get_profiles() -> impl Responder {
    match ProfileService::get_all() {
        Ok(profiles) => {
            HttpResponse::Ok().json(profiles)
        },
        Err(e) => {
            HttpResponse::InternalServerError().body(e)
        },
    }
}

#[derive(Deserialize)]
pub struct CreateProfileRequest {
    name: String,

    #[serde(flatten)]
    data: Profile,
}

pub async fn create_profile(
    request: web::Json<CreateProfileRequest>
) -> impl Responder {
    let CreateProfileRequest { name, data } = request.into_inner();

    match ProfileService::set(&name, &data) {
        Ok(()) => {
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            HttpResponse::InternalServerError().body(e)
        }
    }
}

pub async fn create_profile_alias() -> impl Responder {
    HttpResponse::Ok()
}
