#[derive(Debug, Clone)]
pub enum ASTNode {
    Module {
        name: Box<ASTNode>,
        definitions: Vec<ASTNode>,
    },
    Model {
        name: String,
        text: String,
    },
    EnumerableModel {
        name: String,
        possible_models: Vec<ASTNode>,
    },
    Prompt {
        name: String,
        parameters: Vec<(String, ASTNode, bool)>,
        return_type: Box<ASTNode>,
        text: String,
    },
    CompositePrompt {
        name: String,
        parameters: Vec<(String, ASTNode, bool)>,
        return_type: Box<ASTNode>,
        prompts: Vec<Box<ASTNode>>,
    },
    Function {
        name: String,
        parameters: Vec<(String, ASTNode, bool)>,
        return_type: Box<ASTNode>,
        instructions: Vec<ASTNode>,
    },
    Assignment {
        lhs: String,
        rhs: Box<ASTNode>,
    },
    FunctionCall {
        name: String,
        arguments: Vec<String>,
        return_type_coercion: Option<Box<ASTNode>>,
    },
    ListComprehension {
        expression: Box<ASTNode>,
        variable: String,
        iterable: Box<ASTNode>,
    },
    StringLiteral(String),
    Identifier(String),
    QualifiedIdentifier(Vec<String>),
    ReturnStatement(Box<ASTNode>),
}
