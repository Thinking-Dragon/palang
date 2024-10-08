use std::path::PathBuf;

use palang_virtual_machine::{assembly::{assembly::Assembly, loader::load_assembly}, load_assembly_file};
use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, Clone, Serialize, Deserialize, Tabled)]
#[serde(rename_all = "lowercase")]
pub enum AssemblySource {
    Path(String),
    Code(String),
}

impl AssemblySource {
    pub fn new_remote(path: String) -> Self {
        AssemblySource::Path(path)
    }

    pub fn new_local(code: String) -> Self {
        AssemblySource::Code(code)
    }
}

impl AssemblySource {
    pub fn resolve_assembly(&self) -> Result<Assembly, String> {
        match self {
            AssemblySource::Path(path) => {
                load_assembly_file(&PathBuf::from(path))
            },
            AssemblySource::Code(code) => {
                load_assembly(code)
            },
        }
    }
}
