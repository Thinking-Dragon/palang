use palang_core::{
    language::assembly_source::{
        AssemblySource,
        WrappedAssembly
    },
    project::{
        Project,
        WrappedProject
    },
    storage::NamedData
};

use crate::server_proxy::ServerProxy;

impl ServerProxy {
    pub fn get_projects(&self) -> Result<Vec<NamedData<WrappedProject>>, String> {
        self.get("projects")
    }

    pub fn get_project(&self, project: &String) -> Result<NamedData<WrappedProject>, String> {
        self.get(format!("projects/{}", project).as_str())
    }

    pub fn add_project(&mut self, project: &NamedData<Project>) -> Result<(), String> {
        self.post_only("projects", &project)
    }

    pub fn get_assemblies(&self, project: &String) -> Result<Vec<WrappedAssembly>, String> {
        self.get(format!("projects/{}/assemblies", project).as_str())
    }

    pub fn add_assembly(
        &mut self,
        project: &String,
        assembly: &AssemblySource
    ) -> Result<(), String> {
        self.post_only(
            format!("projects/{}/assemblies", project).as_str(),
            &assembly
        )
    }
}
