use std::fs;

use analyze::semantic_analyzer::analyze_semantics;
use clap::Parser;
use parse::{ast_node::ASTNode, parser::parse};
use tokenize::{tokenizer::tokenize, tokens::Token};

pub mod tokenize;
pub mod parse;
pub mod analyze;

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

    println!("Compiling \"{}\"\n", args.source_file);
    let source_code: String = fs::read_to_string(args.source_file)
        .expect("The file source file does not exist.");

    print!("1. Creating token sequence from source code.");
    let tokens: Vec<Token> = tokenize(&source_code);
    println!(" ✅");

    print!("2. Creating abstract syntax tree from token sequence.");
    let ast: Result<ASTNode, String> = parse(tokens);
    match ast.clone() {
        Ok(_) => {
            println!(" ✅");
        }
        Err(e) => {
            println!(" ❌\n\t{}", e);
            return;
        },
    }

    print!("3. Analyzing abstract syntax tree semantics.");
    match analyze_semantics(&ast.clone().unwrap()) {
        Ok(_) => {
            println!(" ✅");
        },
        Err(e) => {
            println!(" ❌\n\t{}", e);
            return;
        },
    }
}
