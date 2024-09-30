use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

use crate::api::v1::{
    models::assembly::AssemblySource,
    services::{
        project::ProjectService,
        storage::Storable
    }
};

pub async fn get_assemblies(path: web::Path<String>) -> impl Responder {
    let project: String = path.into_inner();

    match ProjectService::get(&project) {
        Ok(project) => {
            HttpResponse::Ok().json(project.assemblies)
        },
        Err(e) => {
            HttpResponse::InternalServerError().body(e)
        },
    }
}

#[derive(Deserialize)]
pub struct CreateAssemblyRequest {
    assembly: AssemblySource,
}

pub async fn create_assembly(
    path: web::Path<String>,
    request: web::Json<CreateAssemblyRequest>,
) -> impl Responder {
    let project: String = path.into_inner();
    let CreateAssemblyRequest { assembly } = request.into_inner();

    match ProjectService::get(&project) {
        Ok(project_data) => {
            let mut project_data = project_data;
            project_data.assemblies.push(assembly);

            match ProjectService::set(&project, &project_data) {
                Ok(()) => {
                    HttpResponse::Ok().finish()
                },
                Err(e) => {
                    HttpResponse::InternalServerError().body(e)
                },
            }
        },
        Err(e) => {
            HttpResponse::InternalServerError().body(e)
        },
    }
}
