use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, Serialize, Deserialize, Tabled)]
pub struct Project {
    pub name: String,
}

impl Project {
    pub fn new(name: String) -> Self {
        Project { name }
    }
}
