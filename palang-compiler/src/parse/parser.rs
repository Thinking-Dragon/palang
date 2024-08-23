use crate::tokenize::tokens::Token;

use super::ast_node::ASTNode;

struct ParserContext {
    tokens: Vec<Token>,
    cursor: usize,
}

impl ParserContext {
    pub fn peek(&self) -> Result<Token, String> {
        if self.cursor < self.tokens.len() {
            Ok(self.tokens[self.cursor].clone())
        }
        else {
            Err("Tried to peek after the end of the tokens list".to_string())
        }
    }

    pub fn next(&mut self) -> Result<Token, String> {
        let next_token = self.peek()?;
        self.cursor += 1;

        Ok(next_token)
    }

    pub fn reached_end(&self) -> bool {
        match self.peek() {
            Ok(_) => false,
            Err(_) => true,
        }
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<ASTNode, String> {
    let mut ctx: ParserContext = ParserContext {
        tokens: tokens,
        cursor: 0
    };
    parse_module(&mut ctx)
}

fn parse_module(ctx: &mut ParserContext) -> Result<ASTNode, String> {
    expect_token(ctx, &Token::Module)?;
    let name: Box<ASTNode> = Box::new(parse_qualified_identifier(ctx)?);
    let mut definitions: Vec<ASTNode> = Vec::new();

    while !ctx.reached_end() {
        match ctx.peek()? {
            Token::Model  => {
                definitions.push(parse_model(ctx)?);
            },
            Token::Prompt => {
                definitions.push(parse_prompt(ctx)?);
            },
            Token::Function => {
                definitions.push(parse_function(ctx)?);
            },
            _ => {
                ctx.next()?;
            },
        }
    }

    Ok(ASTNode::Module { name, definitions })
}

fn parse_model(ctx: &mut ParserContext) -> Result<ASTNode, String> {
    expect_token(ctx, &Token::Model)?;

    let name: String = parse_definition_name(ctx)?;
    let text: String = parse_text_body(ctx)?;

    Ok(ASTNode::Model { name, text })
}

fn parse_prompt(ctx: &mut ParserContext) -> Result<ASTNode, String> {
    expect_token(ctx, &Token::Prompt)?;

    let name: String = parse_definition_name(ctx)?;
    let (parameters, return_type) = parse_parameters(ctx)?;
    let text: String = parse_text_body(ctx)?;

    Ok(ASTNode::Prompt {
        name,
        parameters,
        return_type: Box::new(return_type),
        text,
    })
}

fn parse_function(ctx: &mut ParserContext) -> Result<ASTNode, String> {
    expect_token(ctx, &Token::Function)?;

    let name: String = parse_definition_name(ctx)?;
    let (parameters, return_type) = parse_parameters(ctx)?;
    let instructions: Vec<ASTNode> = parse_instructions(ctx)?;

    Ok(ASTNode::Function {
        name,
        parameters,
        return_type: Box::new(return_type),
        instructions,
    })
}

fn parse_qualified_identifier(ctx: &mut ParserContext) -> Result<ASTNode, String> {
    let mut parts: Vec<String> = Vec::new();

    loop {
        match ctx.peek() {
            Ok(Token::Identifier(part)) => {
                parts.push(part);
                ctx.next()?;
            },
            Ok(Token::DoubleColon) => {
                ctx.next()?;
            },
            _ => break,
        }
    }

    if parts.is_empty() {
        Err("Identifier cannot be empty".to_string())
    }
    else {
        Ok(ASTNode::QualifiedIdentifier(parts))
    }
}

fn parse_identifier(ctx: &mut ParserContext) -> Result<String, String> {
    match ctx.peek()? {
        Token::Identifier(identifier) => {
            ctx.next()?;
            Ok(identifier)
        },
        _ => Err(format!("Expected identifier, got {:?}", ctx.peek()?))
    }
}

fn parse_definition_name(ctx: &mut ParserContext) -> Result<String, String> {
    match ctx.peek()? {
        Token::Identifier(name) => {
            ctx.next()?;
            Ok(name)
        },
        _ => Err(format!("Expected a name, found {:?}", ctx.peek()?)),
    }
}

fn parse_parameters(ctx: &mut ParserContext) -> Result<(Vec<(String, ASTNode, bool)>, ASTNode), String> {
    expect_token(ctx, &Token::OpenParenthesis)?;

    let mut parameters: Vec<(String, ASTNode, bool)> = Vec::new();

    while ctx.peek()? != Token::CloseParenthesis {
        match ctx.peek()? {
            Token::Identifier(parameter_name) => {
                ctx.next()?;
                expect_token(ctx, &Token::Colon)?;
                let parameter_type = parse_qualified_identifier(ctx)?;
                parameters.push((parameter_name, parameter_type, false));
            },
            Token::Comma => {
                ctx.next()?;
            }
            Token::OpenBracket => {
                ctx.next()?;
                expect_token(ctx, &Token::CloseBracket)?;
                parameters.last_mut().unwrap().2 = true;
            }
            _ => {
                return Err(format!("Unexpected in parameters signature: {:?}", ctx.peek()?));
            }
        }
    }

    ctx.next()?;

    expect_token(ctx, &Token::Arrow)?;

    let return_type: ASTNode = parse_qualified_identifier(ctx)?;

    Ok((parameters, return_type))
}

fn parse_text_body(ctx: &mut ParserContext) -> Result<String, String> {
    match ctx.peek()? {
        Token::StringLiteral(text) => {
            ctx.next()?;
            Ok(text)
        }
        _ => Err(format!("Expected text, found {:?}", ctx.peek()?))
    }
}

fn parse_instructions(ctx: &mut ParserContext) -> Result<Vec<ASTNode>, String> {
    expect_token(ctx, &Token::OpenBrace)?;

    let mut instructions: Vec<ASTNode> = Vec::new();

    while ctx.peek()? != Token::CloseBrace {
        instructions.push(parse_statement(ctx)?);
    }

    Ok(instructions)
}

fn parse_statement(ctx: &mut ParserContext) -> Result<ASTNode, String> {
    match ctx.peek()? {
        Token::Identifier(_) => parse_assignment_or_function_call(ctx),
        Token::Return        => parse_return_statement(ctx),
        _ => Err(format!("Unexpected token in statement {:?}", ctx.peek()?)),
    }
}

fn parse_assignment_or_function_call(ctx: &mut ParserContext) -> Result<ASTNode, String> {
    let identifier: String = parse_identifier(ctx)?;

    match ctx.peek()? {
        Token::Equal => {
            ctx.next()?;
            let value: ASTNode = parse_statement(ctx)?;
            Ok(ASTNode::Assignment { lhs: identifier, rhs: Box::new(value) })
        },
        Token::OpenParenthesis => {
            ctx.next()?;
            let arguments: Vec<String> = parse_arguments(ctx)?;
            Ok(ASTNode::FunctionCall { name: identifier, arguments })
        },
        _ => Ok(ASTNode::Identifier(identifier)),
    }
}

fn parse_arguments(ctx: &mut ParserContext) -> Result<Vec<String>, String> {
    let mut arguments: Vec<String> = Vec::new();

    while ctx.peek()? != Token::CloseParenthesis {
        match ctx.peek()? {
            Token::Identifier(argument) => {
                ctx.next()?;
                arguments.push(argument)
            },
            Token::Comma => {
                ctx.next()?;
            },
            _ => {
                return Err(format!("Unexpected token in function call arguments {:?}", ctx.peek()?));
            },
        }
    }
    ctx.next()?;

    Ok(arguments)
}

fn parse_return_statement(ctx: &mut ParserContext) -> Result<ASTNode, String> {
    expect_token(ctx, &Token::Return)?;
    Ok(ASTNode::ReturnStatement(Box::new(parse_statement(ctx)?)))
}

fn expect_token(ctx: &mut ParserContext, expected_token: &Token) -> Result<(), String> {
    if ctx.peek()? == *expected_token {
        ctx.next()?;
        Ok(())
    }
    else {
        Err(format!("Expected token {:?}, found {:?}", expected_token, ctx.peek()?))
    }
}
