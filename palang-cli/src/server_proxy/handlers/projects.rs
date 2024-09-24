use crate::server_proxy::{models::{assembly::Assembly, project::Project}, ServerProxy};


impl ServerProxy {
    pub fn get_projects(&self) -> Vec<Project> {
        Vec::new()
    }

    pub fn add_project(&mut self, project: &Project) {

    }

    pub fn get_assemblies(&self) -> Vec<Assembly> {
        Vec::new()
    }

    pub fn add_assembly(&mut self, assembly: &Assembly) {

    }
}
