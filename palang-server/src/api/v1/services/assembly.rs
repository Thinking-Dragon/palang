use crate::api::v1::models::assembly::Assembly;

use super::storage::{load, load_all, name_data, store, NamedData, Storable};

pub struct AssemblyService;

impl AssemblyService {
    pub fn get_from_project(assembly: &String, project: &String) -> Result<Assembly, String> {
        Self::get(&assembly_and_project_to_name(assembly, project))
    }

    pub fn get_all_from_project(project: &String) -> Result<Vec<NamedData<Assembly>>, String> {
        let all_assemblies: Vec<NamedData<Assembly>> = Self::get_all()?;

        Ok(
            all_assemblies
                .into_iter()
                .filter(|asm| asm.name.starts_with(&format!("{}:", project)))
                .map(|asm| {
                    let assembly_name: String = asm.name[project.len() + 1..].to_string();
                    name_data(assembly_name, asm.data)
                })
                .collect()
        )
    }

    pub fn set_for_project(assembly: &String, project: &String, data: &Assembly) -> Result<(), String> {
        Self::set(&assembly_and_project_to_name(assembly, project), data)
    }
}

impl Storable<Assembly> for AssemblyService {
    fn get(name: &String) -> Result<Assembly, String> {
        load(name, "assemblies")
    }

    fn get_all() -> Result<Vec<NamedData<Assembly>>, String> {
        load_all("assemblies")
    }

    fn set(name: &String, assembly: &Assembly) -> Result<(), String> {
        store(name, "assemblies", assembly)
    }
}

fn assembly_and_project_to_name(assembly: &String, project: &String) -> String {
    format!("{}:{}", assembly, project)
}
