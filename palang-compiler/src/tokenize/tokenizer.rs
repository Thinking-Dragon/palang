use std::{iter::Peekable, str::Chars};

use super::tokens::Token;

pub fn tokenize<'a>(input: &'a str) -> Vec<Token> {
    let mut chars = input.chars().peekable();

    let mut tokens = Vec::new();
    let mut braces_are_text = false;

    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\t' | '\n' | '\r' => {
                chars.next();
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let identifier = tokenize_identifier(&mut chars);

                if identifier == Token::Prompt || identifier == Token::Model {
                    braces_are_text = true;
                }

                tokens.push(identifier);
            }
            '"' => {
                tokens.push(tokenize_string_literal(&mut chars));
            }
            '-' => {
                chars.next();
                if chars.peek() == Some(&'>') {
                    chars.next();
                    tokens.push(Token::Arrow);
                } else {
                    tokens.push(Token::Minus);
                }
            }
            ':' => {
                chars.next();
                if chars.peek() == Some(&':') {
                    chars.next();
                    tokens.push(Token::DoubleColon);
                } else {
                    tokens.push(Token::Colon);
                }
            }
            '{' => {
                if braces_are_text {
                    let mut text: String = String::new();
                    let mut brace_counter: u32 = 0;

                    chars.next();
                    while let Some(&c) = chars.peek() {
                        if c == '{' {
                            brace_counter += 1;
                        }
                        if c == '}' {
                            if brace_counter == 0 {
                                break;
                            }
                            else {
                                brace_counter -= 1;
                            }
                        }
                        text.push(c);
                        chars.next();
                    }

                    tokens.push(Token::StringLiteral(text.to_string()));
                    chars.next();

                    braces_are_text = false;
                }
                else {
                    chars.next();
                    tokens.push(Token::OpenBrace);
                }
            }
            '}' => {
                chars.next();
                tokens.push(Token::CloseBrace);
            }
            '[' => {
                chars.next();
                tokens.push(Token::OpenBracket);
            }
            ']' => {
                chars.next();
                tokens.push(Token::CloseBracket);
            }
            '(' => {
                chars.next();
                tokens.push(Token::OpenParenthesis);
            }
            ')' => {
                chars.next();
                tokens.push(Token::CloseParenthesis);
            }
            ',' => {
                chars.next();
                tokens.push(Token::Comma);
            }
            '=' => {
                chars.next();
                tokens.push(Token::Equal);
            }
            '.' => {
                chars.next();
                tokens.push(Token::Dot);
            }
            '@' => {
                chars.next();
                tokens.push(Token::At);
            }
            '+' => {
                chars.next();
                tokens.push(Token::Plus);
            }
            '*' => {
                chars.next();
                tokens.push(Token::Times);
            }
            '/' => {
                chars.next();
                tokens.push(Token::Division);
            }
            '%' => {
                chars.next();
                tokens.push(Token::Modulo);
            }
            '&' => {
                chars.next();
                tokens.push(Token::BitAnd);
            }
            '|' => {
                chars.next();
                tokens.push(Token::BitOr);
            }
            _ => {
                // Handle unexpected characters or errors
                chars.next();
            }
        }
    }

    tokens
}

fn tokenize_identifier<'a>(chars: &mut Peekable<Chars<'a>>) -> Token {
    let mut identifier = String::new();
    while let Some(&c) = chars.peek() {
        if c.is_alphanumeric() || c == '_' {
            identifier.push(chars.next().unwrap());
        } else {
            break;
        }
    }
    match identifier.as_str() {
        "module"   => Token::Module,
        "model"    => Token::Model,
        "prompt"   => Token::Prompt,
        "function" => Token::Function,
        "return"   => Token::Return,
        "for"      => Token::For,
        "in"       => Token::In,
        "&&"       => Token::And,
        "||"       => Token::Or,
        _ => Token::Identifier(identifier),
    }
}

fn tokenize_string_literal<'a>(chars: &mut Peekable<Chars<'a>>) -> Token {
    chars.next(); // Consume opening quote
    let mut string = String::new();
    while let Some(&c) = chars.peek() {
        if c == '"' {
            chars.next();
            break;
        }
        string.push(chars.next().unwrap());
    }
    Token::StringLiteral(string)
}
