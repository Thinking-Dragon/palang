use serde::{Deserialize, Serialize};

use super::assembly::AssemblySource;

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub assemblies: Vec<AssemblySource>,
}

impl Project {
    pub fn new() -> Self {
        Project { assemblies: Vec::new() }
    }
}
