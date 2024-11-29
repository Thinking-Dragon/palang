use actix_web::{web::{self, Data}, App, HttpServer};
use crate::api::v1::{
    routes::{
        assemblies,
        profiles,
        projects,
        tasks
    },
    server::AppState
};

#[actix_web::main]
pub async fn start_server(host: String, port: u16) -> std::io::Result<()> {
    let state: Data<AppState> = web::Data::new(AppState::new());

    println!("Starting server at http://{}:{}", host, port);

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(
                web::scope("/api/v1")
                    .route("/run/{task:.*}", web::post().to(tasks::run_task))
                    .route("/projects", web::get().to(projects::get_projects))
                    .route("/projects", web::post().to(projects::create_project))
                    .route("/projects/{project}", web::get().to(projects::get_project))
                    .route("/projects/{project}/assemblies", web::get().to(assemblies::get_assemblies))
                    .route("/projects/{project}/assemblies", web::post().to(assemblies::create_assembly))
                    .route("/profiles", web::get().to(profiles::get_profiles))
                    .route("/profiles", web::post().to(profiles::create_profile))
                    .route("/profiles/alias", web::post().to(profiles::create_profile_alias))
            )
    })
    .bind((host, port))?
    .run()
    .await
}
