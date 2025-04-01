use crate::parse::ast_node::ASTNode;

#[derive(Debug, Clone)]
pub struct ParameterInfo {
    pub name: String,
    pub full_type: String,
    pub is_array: bool,
}

impl ParameterInfo {
    pub fn new(name: String, full_type: ASTNode, is_array: bool) -> Result<Self, String> {
        match full_type {
            ASTNode::QualifiedIdentifier(parts) => {
                Ok(
                    ParameterInfo {
                        name,
                        full_type: parts.join("/").to_lowercase(),
                        is_array,
                    }
                )
            },
            _ => Err(format!("Unexpected parameter type: {:?}", full_type))
        }
    }
}
