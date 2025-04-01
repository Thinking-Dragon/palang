use std::ops::Deref;

use rand::{distributions::Alphanumeric, Rng};

use crate::parse::ast_node::ASTNode;

struct CodeGenerationContext {
    generated_assembly: String,
    module_fully_qualified_name: Vec<String>,
}

impl CodeGenerationContext {
    pub fn new() -> Self {
        CodeGenerationContext {
            generated_assembly: String::new(),
            module_fully_qualified_name: Vec::new()
        }
    }
}

pub fn generate_palassembly(ast: &ASTNode) -> Result<String, String> {
    let mut ctx: CodeGenerationContext = CodeGenerationContext::new();
    generate_node(&mut ctx, ast)?;

    Ok(ctx.generated_assembly.clone())
}

fn generate_node(ctx: &mut CodeGenerationContext, node: &ASTNode) -> Result<(), String> {
    match node {
        ASTNode::Module {
            name,
            definitions
        } => {
            generate_module(ctx, name, definitions)
        },
        ASTNode::Model {
            name,
            text
        } => {
            generate_model(ctx, name, text)
        },
        ASTNode::EnumerableModel {
            name,
            possible_models
        } => {
            generate_enumerable_model(ctx, name, possible_models)
        },
        ASTNode::Prompt {
            name,
            parameters,
            return_type,
            text
        } => {
            generate_prompt(ctx, name, parameters, return_type, text)
        },
        ASTNode::Function {
            name,
            parameters,
            return_type,
            instructions
        } => {
            generate_function(ctx, name, parameters, return_type, instructions)
        },
        _ => Err(format!("Unexpected node type: {:?}", node)),
    }
}

fn generate_module(
    ctx: &mut CodeGenerationContext,
    name: &ASTNode,
    definitions: &[ASTNode]
) -> Result<(), String> {
    if let ASTNode::QualifiedIdentifier(parts) = name {
        ctx.module_fully_qualified_name = parts.clone();
        ctx.generated_assembly.push_str(&format!("MODULE {}\n", parts.join("/")));
    } else {
        return Err("Invalid module name".to_string());
    }

    for definition in definitions {
        generate_node(ctx, definition)?;
        ctx.generated_assembly.push('\n');
    }

    Ok(())
}

fn generate_model(
    ctx: &mut CodeGenerationContext,
    name: &str,
    text: &str
) -> Result<(), String> {
    let full_name: String = get_full_name(ctx, name);

    ctx.generated_assembly.push_str(
        &format!(
            "MODEL {}\nSTART\n{}\nEND",
            full_name,
            remove_indentation(text).trim(),
        )
    );

    Ok(())
}

fn generate_enumerable_model(
    ctx: &mut CodeGenerationContext,
    name: &str,
    possible_models: &Vec<ASTNode>,
) -> Result<(), String> {
    let full_name: String = get_full_name(ctx, name);
    let possible_models: String = possible_models.iter()
                                                 .map(|model_name| get_type_name(ctx, &model_name).ok())
                                                 .filter(|model_name| model_name.is_some())
                                                 .map(|model_name| model_name.unwrap())
                                                 .collect::<Vec<String>>()
                                                 .join(" ");

    ctx.generated_assembly.push_str(
        &format!(
            "ENUM_MODEL {} {}",
            full_name,
            possible_models,
        )
    );

    Ok(())
}

fn generate_prompt(
    ctx: &mut CodeGenerationContext,
    name: &str,
    parameters: &[(String, ASTNode, bool)],
    return_type: &ASTNode, text: &str
) -> Result<(), String> {
    let full_name: String = get_full_name(ctx, name);
    let args: String = parameters.iter()
                                 .map(|(name, _, _)| name.clone())
                                 .collect::<Vec<_>>()
                                 .join(" ");
    let ret_type: String = get_type_name(ctx, return_type)?;

    ctx.generated_assembly.push_str(
        &format!(
            "PROMPT {}\nARGUMENTS {}\nRETURNS {}\nSTART\n{}\nEND",
            full_name,
            args,
            ret_type,
            remove_indentation(text).trim(),
        )
    );

    Ok(())
}

fn generate_function(
    ctx: &mut CodeGenerationContext,
    name: &str,
    parameters: &[(String, ASTNode, bool)],
    return_type: &ASTNode,
    instructions: &[ASTNode]
) -> Result<(), String> {
    let full_name: String = get_full_name(ctx, name);
    let args: String = parameters.iter().map(|(name, _, _)| name.clone())
                                        .collect::<Vec<_>>()
                                        .join(" ");
    let ret_type: String = get_type_name(ctx, return_type)?;

    ctx.generated_assembly.push_str(
        &format!("FUNCTION {}\nARGUMENTS {}\nRETURNS {}\nSTART\n",
            full_name,
            args,
            ret_type
        )
    );

    for instruction in instructions {
        generate_instruction(ctx, instruction)?;
    }

    ctx.generated_assembly.push_str("END");
    
    Ok(())
}

fn generate_instruction(
    ctx: &mut CodeGenerationContext,
    instruction: &ASTNode
) -> Result<(), String> {
    match instruction {
        ASTNode::Assignment { lhs, rhs } => {
            match rhs.deref() {
                ASTNode::FunctionCall { name, arguments, return_type_coercion } => {
                    generate_invoke_function(ctx, name, arguments, return_type_coercion)?;
                    ctx.generated_assembly.push_str(&format!("ASSIGN {} @invocation_registry", lhs));
                },
                _ => {
                    ctx.generated_assembly.push_str(&format!("ASSIGN {} ", lhs));
                    generate_expression(ctx, rhs)?;
                },
            }
            ctx.generated_assembly.push('\n');
        },
        ASTNode::FunctionCall { name, arguments, return_type_coercion } => {
            generate_invoke_function(ctx, name, arguments, return_type_coercion)?
        },
        ASTNode::ReturnStatement(expr) => {
            match expr.deref() {
                ASTNode::FunctionCall { name, arguments, return_type_coercion } => {
                    let anonymous_variable_name: String = generate_anonymous_variable_name();
                    generate_invoke_function(ctx, name, arguments, return_type_coercion)?;
                    ctx.generated_assembly.push_str(
                        &format!(
                            "ASSIGN {} @invocation_registry\n",
                            anonymous_variable_name,
                        )
                    );
                    ctx.generated_assembly.push_str(
                        &format!(
                            "RETURN {}",
                            anonymous_variable_name,
                        )
                    )
                },
                _ => {
                    ctx.generated_assembly.push_str("RETURN ");
                    generate_expression(ctx, expr)?;
                }
            }
            ctx.generated_assembly.push('\n');
        },
        _ => return Err(format!("Unsupported instruction: {:?}", instruction)),
    }
    Ok(())
}

fn generate_expression(
    ctx: &mut CodeGenerationContext,
    expr: &ASTNode
) -> Result<(), String> {
    match expr {
        ASTNode::Identifier(name) => {
            ctx.generated_assembly.push_str(name);
        },
        ASTNode::StringLiteral(value) => {
            ctx.generated_assembly.push_str(&format!("\"{}\"", value));
        },
        ASTNode::FunctionCall { name, arguments, return_type_coercion } => {
            generate_invoke_function(ctx, name, arguments, return_type_coercion)?;
        },
        ASTNode::ListComprehension { expression, variable, iterable } => {
            ctx.generated_assembly.push_str("[");
            generate_expression(ctx, expression)?;
            ctx.generated_assembly.push_str(&format!(" for {} in ", variable));
            generate_expression(ctx, iterable)?;
            ctx.generated_assembly.push_str("]");
        },
        _ => return Err(format!("Unsupported expression: {:?}", expr)),
    }
    Ok(())
}

fn get_full_name(
    ctx: &CodeGenerationContext,
    name: &str
) -> String {
    let mut full_name = ctx.module_fully_qualified_name.join("/");

    if !full_name.is_empty() {
        full_name.push('/');
    }
    full_name.push_str(name);

    full_name
}

fn get_type_name(
    ctx: &CodeGenerationContext,
    type_node: &ASTNode
) -> Result<String, String> {
    match type_node {
        ASTNode::QualifiedIdentifier(parts) => {
            if parts.len() == 1 {
                Ok(get_full_name(ctx, parts.first().unwrap()))
            }
            else {
                Ok(parts.join("/"))
            }
        },
        ASTNode::Identifier(name) => Ok(get_full_name(ctx, name)),
        _ => Err(format!("Invalid type: {:?}", type_node)),
    }
}

fn get_full_type_name(
    ctx: &CodeGenerationContext,
    type_name: &String
) -> Result<String, String> {
    let parts: Vec<String> = type_name.split("/").map(str::to_string).collect();
    if parts.len() == 1 {
        Ok(get_full_name(ctx, &type_name))
    }
    else {
        Ok(type_name.to_string())
    }
}

fn generate_invoke_function(
    ctx: &mut CodeGenerationContext,
    name: &String,
    arguments: &Vec<String>,
    return_type_coercion: &Option<Box<ASTNode>>,
) -> Result<(), String> {
    if let Some(coerced_return_type) = return_type_coercion {
        ctx.generated_assembly.push_str(
            &format!(
                "INVOKE {} {}\nCOERCE_RETURN {}\n",
                get_full_type_name(ctx, name)?,
                arguments.join(" "),
                get_type_name(ctx, coerced_return_type.deref())?,
            )
        );
    }
    else {
        ctx.generated_assembly.push_str(
            &format!(
                "INVOKE {} {}\n",
                get_full_type_name(ctx, name)?,
                arguments.join(" "),
            )
        );
    }

    Ok(())
}

fn remove_indentation(text: &str) -> String {
    let lines: Vec<&str> = text.lines().collect();

    let lowest_indent = lines.iter()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.len() - line.trim_start().len())
        .min()
        .unwrap_or(0);

    lines.into_iter().map(|line| {
        if line.len() > lowest_indent {
            &line[lowest_indent..]
        }
        else {
            line.trim_start()
        }
    }).collect::<Vec<&str>>().join("\n")
}

fn generate_anonymous_variable_name() -> String {
    format!(
        "var_{}",
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(12)
            .map(char::from)
            .collect::<String>()
    )
}
