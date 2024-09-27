use crate::api::v1::models::project::Project;

use super::storage::{load, load_all, store, NamedData, Storable};

pub struct ProjectService;

impl Storable<Project> for ProjectService {
    fn get(name: &String) -> Result<Project, String> {
        load(name, "projects")
    }

    fn get_all() -> Result<Vec<NamedData<Project>>, String> {
        load_all("projects")
    }

    fn set(name: &String, project: &Project) -> Result<(), String> {
        store(name, "projects", project)
    }
}
