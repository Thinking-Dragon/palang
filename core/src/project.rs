use serde::{Deserialize, Serialize};

use crate::language::assembly_source::{AssemblySource, WrappedAssembly};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub assemblies: Vec<AssemblySource>,
}

impl Project {
    pub fn new() -> Self {
        Project { assemblies: Vec::new() }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WrappedProject {
    pub assemblies: Vec<WrappedAssembly>,
}

impl WrappedProject {
    pub fn new() -> Self {
        WrappedProject { assemblies: Vec::new() }
    }

    pub fn from_project(project: Project) -> Self {
        WrappedProject {
            assemblies: project.assemblies.iter()
                .filter_map(
                    |assembly|
                    match assembly.resolve_assembly() {
                        Ok(wrapped_assembly) => Some(wrapped_assembly),
                        Err(_) => None,
                    }
                )
                .collect()
        }
    }
}
