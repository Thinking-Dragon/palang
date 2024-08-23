use std::fs;

use clap::Parser;
use parse::{parser::parse, ast_node::ASTNode};
use tokenize::{tokenizer::tokenize, tokens::Token};

pub mod tokenize;
pub mod parse;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    source_file: String,

    #[arg(short, long)]
    target_file: String,
}

fn main() {
    let args: Args = Args::parse();
    let source_code: String = fs::read_to_string(args.source_file)
            .expect("The file source file does not exist.");
    let tokens: Vec<Token> = tokenize(&source_code);
    println!("{:?}", tokens);

    println!("---");

    let ast: Result<ASTNode, String> = parse(tokens);
    println!("{:?}", ast);
}
