use core::fmt;

use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, Serialize, Deserialize, Tabled)]
pub struct Assembly {
    pub project: String,
    pub assembly: String,
    pub content: AssemblyContent,
}

#[derive(Debug,  Serialize, Deserialize, Tabled)]
#[serde(rename_all = "lowercase")]
pub enum AssemblyContent {
    Path(String),
    Code(String),
}

impl Assembly {
    pub fn new_remote(
        project: String,
        assembly: String,
        path: String,
    ) -> Self {
        Assembly { project, assembly, content: AssemblyContent::Path(path) }
    }

    pub fn new_local(
        project: String,
        assembly: String,
        code: String,
    ) -> Self {
        Assembly { project, assembly, content: AssemblyContent::Code(code) }
    }
}

impl fmt::Display for AssemblyContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssemblyContent::Path(path) => write!(f, "{}", path),
            AssemblyContent::Code(code) => write!(f, "{}", code),
        }
    }
}
