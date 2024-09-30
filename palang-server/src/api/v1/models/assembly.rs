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
