use actix_web::{
    web,
    HttpResponse,
    Responder
};

use palang_core::{
    profile::Profile,
    services::profile::ProfileService,
    storage::Storable
};

use serde::Deserialize;

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
