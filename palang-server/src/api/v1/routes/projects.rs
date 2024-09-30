use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

use crate::api::v1::{
    models::project::Project,
    services::{
        project::ProjectService,
        storage::{name_data, Storable}
    }
};

pub async fn get_projects() -> impl Responder {
    match ProjectService::get_all() {
        Ok(projects) => {
            HttpResponse::Ok().json(projects)
        },
        Err(e) => {
            HttpResponse::InternalServerError().body(e)
        },
    }
}

pub async fn get_project(path: web::Path<String>) -> impl Responder {
    let project: String = path.into_inner();

    match ProjectService::get(&project) {
        Ok(project_data) => {
            HttpResponse::Ok().json(name_data(project, project_data))
        },
        Err(e) => {
            HttpResponse::InternalServerError().body(e)
        },
    }
}

#[derive(Deserialize)]
pub struct CreateProjectRequest {
    name: String,

    #[serde(flatten)]
    data: Project,
}

pub async fn create_project(
    request: web::Json<CreateProjectRequest>
) -> impl Responder {
    let CreateProjectRequest { name, data } = request.into_inner();

    match ProjectService::set(&name, &data) {
        Ok(()) => {
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            HttpResponse::InternalServerError().body(e)
        },
    }
}
