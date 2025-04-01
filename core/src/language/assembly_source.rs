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

#[derive(Debug, Serialize, Deserialize)]
pub struct Assembly {
    pub instructions: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WrappedAssembly {
    #[serde(flatten)]
    pub source: AssemblySource,
    pub instructions: String,
}

impl WrappedAssembly {
    pub fn new(source: AssemblySource, instructions: String) -> Self {
        WrappedAssembly { source, instructions }
    }
}

impl AssemblySource {
    pub fn resolve_assembly(&self) -> Result<WrappedAssembly, String> {
        match self {
            AssemblySource::Path(path) => {
                let assembly: Assembly = reqwest::blocking::get(path)
                    .map_err(|e| format!("{:?}", e))?
                    .json()
                    .map_err(|e| format!("{:?}", e))?;

                Ok(WrappedAssembly::new(self.clone(), assembly.instructions))
            },
            AssemblySource::Code(code) => {
                Ok(WrappedAssembly::new(self.clone(), code.clone()))
            },
        }
    }
}
