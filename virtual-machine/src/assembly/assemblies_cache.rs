use std::collections::HashMap;

use super::{assembly::Assembly, model::Model, task::Task};

pub struct AssembliesCache {
    assemblies: HashMap<String, Assembly>,
    models_index: HashMap<String, String>,
    prompts_index: HashMap<String, String>,
    functions_index: HashMap<String, String>,
}

impl AssembliesCache {
    pub fn new() -> Self {
        AssembliesCache {
            assemblies: HashMap::new(),
            models_index: HashMap::new(),
            prompts_index: HashMap::new(),
            functions_index: HashMap::new(),
        }
    }

    pub fn get_model(&self, model: &String) -> Option<Model> {
        if let Some(assembly_name) = self.models_index.get(model) {
            if let Some(assembly) = self.assemblies.get(assembly_name) {
                return match assembly.models.get(model) {
                    Some(model) => Some(model.clone()),
                    None => None,
                };
            }
        }

        return None;
    }

    pub fn get_task(&self, task: &String) -> Option<Task> {
        if let Some(assembly_name) = self.prompts_index.get(task) {
            if let Some(assembly) = self.assemblies.get(assembly_name) {
                return match assembly.prompts.get(task) {
                    Some(prompt) => Some(Task::Prompt(prompt.clone())),
                    None => None,
                };
            }
        }

        if let Some(assembly_name) = self.functions_index.get(task) {
            if let Some(assembly) = self.assemblies.get(assembly_name) {
                return match assembly.functions.get(task) {
                    Some(function) => Some(Task::Function(function.clone())),
                    None => None,
                };
            }
        }

        return None;
    }

    pub fn load(&mut self, assembly: &Assembly) {
        self.assemblies.insert(assembly.name.clone(), assembly.clone());

        for (key, _) in &assembly.models {
            self.models_index.insert(key.clone(), assembly.name.clone());
        }

        for (key, _) in &assembly.prompts {
            self.prompts_index.insert(key.clone(), assembly.name.clone());
        }

        for (key, _) in &assembly.functions {
            self.functions_index.insert(key.clone(), assembly.name.clone());
        }
    }
}
