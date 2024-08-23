#[derive(Debug)]
pub enum ASTNode {
    Module {
        name: Box<ASTNode>,
        definitions: Vec<ASTNode>,
    },
    Model {
        name: String,
        text: String,
    },
    Prompt {
        name: String,
        parameters: Vec<(String, ASTNode, bool)>,
        return_type: Box<ASTNode>,
        text: String,
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
