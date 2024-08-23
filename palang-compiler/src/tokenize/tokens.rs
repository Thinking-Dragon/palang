#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Module,
    Model,
    Prompt,
    Function,
    Arrow,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    OpenParenthesis,
    CloseParenthesis,
    Colon,
    DoubleColon,
    Comma,
    Equal,
    Dot,
    At,
    Identifier(String),
    StringLiteral(String),
    Return,
    For,
    In,
    Plus,
    Minus,
    Times,
    Division,
    Modulo,
    And,
    Or,
    BitAnd,
    BitOr,
}
