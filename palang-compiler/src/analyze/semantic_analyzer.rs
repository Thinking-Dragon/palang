use std::collections::HashMap;
use crate::parse::ast_node::ASTNode;

use super::{
    function_info::FunctionInfo,
    model_info::ModelInfo,
    parameter_info::ParameterInfo,
    prompt_info::PromptInfo
};

struct SemanticAnalysisContext {
    models: HashMap<String, ModelInfo>,
    prompts: HashMap<String, PromptInfo>,
    functions: HashMap<String, FunctionInfo>,
    module_fully_qualified_name: String,
}

impl SemanticAnalysisContext {
    pub fn new() -> Self {
        SemanticAnalysisContext {
            models: HashMap::new(),
            prompts: HashMap::new(),
            functions: HashMap::new(),
            module_fully_qualified_name: String::new(),
        }
    }

    pub fn register_model(&mut self, name: String) -> Result<(), String> {
        let model_already_known: bool = self.models.contains_key(&name);

        if model_already_known {
            Err(format!("Duplicate model definition for \"{}\"", name))
        }
        else {
            self.models.insert(name, ModelInfo);
            Ok(())
        }
    }

    pub fn register_prompt(&mut self, name: String, parameters: Vec<ParameterInfo>, return_type: String) -> Result<(), String> {
        let prompt_already_known: bool = self.prompts.contains_key(&name);

        if prompt_already_known {
            Err(format!("Duplicate prompt definition for \"{}\"", name))
        }
        else {
            self.prompts.insert(name, PromptInfo { parameters, return_type });
            Ok(())
        }
    }

    pub fn register_function(&mut self, name: String, parameters: Vec<ParameterInfo>, return_type: String) -> Result<(), String> {
        let function_already_known: bool = self.functions.contains_key(&name);

        if function_already_known {
            Err(format!("Duplicate function definition for \"{}\"", name))
        }
        else {
            self.functions.insert(name, FunctionInfo { parameters, return_type });
            Ok(())
        }
    }
}

pub fn analyze_semantics(ast: &ASTNode) -> Result<(), String> {
    let mut ctx: SemanticAnalysisContext = SemanticAnalysisContext::new();

    match ast {
        ASTNode::Module {
            name,
            definitions
        } => analyze_module(&mut ctx, name, definitions),
        _ => Err("Expected module at top level".to_string()),
    }
}

fn analyze_module(ctx: &mut SemanticAnalysisContext, name: &ASTNode, definitions: &[ASTNode]) -> Result<(), String> {
    if let ASTNode::QualifiedIdentifier(parts) = name {
        ctx.module_fully_qualified_name = parts.join("/").to_lowercase();
    } else {
        return Err("Invalid module name".to_string());
    }

    for definition in definitions {
        match definition {
            ASTNode::Model { name, text: _ } => {
                analyze_model(ctx, name)
            },
            ASTNode::Prompt { name, parameters, return_type, text: _ } => {
                analyze_prompt(ctx, name, parameters, return_type)
            },
            ASTNode::Function { name, parameters, return_type, instructions } => {
                analyze_function(ctx, name, parameters, return_type, instructions)
            },
            _ => return Err(format!("Unexpected definition in module: {:?}", definition)),
        }?;
    }

    Ok(())
}

fn analyze_model(ctx: &mut SemanticAnalysisContext, name: &str) -> Result<(), String> {
    let full_name = get_full_name(ctx, name);
    ctx.register_model(full_name)?;

    Ok(())
}

fn analyze_prompt(ctx: &mut SemanticAnalysisContext, name: &str, parameters: &Vec<(String, ASTNode, bool)>, return_type: &ASTNode) -> Result<(), String> {
    let full_name: String = get_full_name(ctx, name);
    let parameter_infos: Vec<ParameterInfo> = extract_parameters(parameters)?;
    let full_return_type: String = get_type_name(return_type)?;

    ctx.register_prompt(full_name, parameter_infos, full_return_type)?;

    Ok(())
}

fn analyze_function(ctx: &mut SemanticAnalysisContext, name: &str, parameters: &Vec<(String, ASTNode, bool)>, return_type: &ASTNode, _instructions: &Vec<ASTNode>) -> Result<(), String> {
    let full_name: String = get_full_name(ctx, name);
    let parameter_infos: Vec<ParameterInfo> = extract_parameters(parameters)?;
    let full_return_type: String = get_type_name(return_type)?;

    ctx.register_function(full_name, parameter_infos.clone(), full_return_type.clone())?;

    // TODO: Analyze the instructions.

    Ok(())
}

fn get_full_name(ctx: &SemanticAnalysisContext, name: &str) -> String {
    let mut full_name: String = ctx.module_fully_qualified_name.clone();

    if !full_name.is_empty() {
        full_name.push_str("/");
    }
    full_name.push_str(name);

    full_name.to_lowercase()
}

fn get_type_name(type_node: &ASTNode) -> Result<String, String> {
    match type_node {
        ASTNode::QualifiedIdentifier(parts) => Ok(parts.join("/").to_lowercase()),
        ASTNode::Identifier(name) => Ok(name.clone()),
        _ => Err(format!("Invalid type: {:?}", type_node)),
    }
}

fn extract_parameters(raw_parameters: &Vec<(String, ASTNode, bool)>) -> Result<Vec<ParameterInfo>, String> {
    raw_parameters.iter()
                  .map(|(name, full_type, is_array)| ParameterInfo::new(
                      name.to_string(),
                      full_type.clone(),
                      is_array.clone()
                  )).collect()
}
