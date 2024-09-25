use crate::server_proxy::{models::{assembly::Assembly, project::Project}, ServerProxy};

impl ServerProxy {
    pub fn get_projects(&self) -> Result<Vec<Project>, String> {
        self.get("projects")
    }

    pub fn add_project(&mut self, project: &Project) -> Result<(), String> {
        self.post("projects", &project)
    }

    pub fn get_assemblies(&self, project: &String) -> Result<Vec<Assembly>, String> {
        self.get(format!("projects/{}/assemblies", project).as_str())
    }

    pub fn add_assembly(
        &mut self,
        project: &String,
        assembly: &Assembly
    ) -> Result<(), String> {
        self.post(
            format!("projects/{}/assemblies", project).as_str(),
            &assembly
        )
    }
}
