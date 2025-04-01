use std::collections::HashMap;

use serde::Serialize;

use super::{
    model::Model,
    prompt::Prompt,
    function::Function,
};

#[derive(Debug, Clone, Serialize)]
pub struct Assembly {
    pub name: String,
    pub models: HashMap<String, Model>,
    pub prompts: HashMap<String, Prompt>,
    pub functions: HashMap<String, Function>
}

impl Assembly {
    pub fn new() -> Self {
        Assembly {
            name: String::new(),
            models: HashMap::new(),
            prompts: HashMap::new(),
            functions: HashMap::new(),
        }
    }
}
