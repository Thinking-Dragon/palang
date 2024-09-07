use std::collections::HashMap;

use super::{function::Function, model::Model, prompt::Prompt};

#[derive(Debug, Clone)]
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
