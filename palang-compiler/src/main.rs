use std::fs;

use clap::Parser;
use tokenize::tokenizer::tokenize;

pub mod tokenize;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    source_file: String,

    #[arg(short, long)]
    target_file: String,
}

fn main() {
    let args = Args::parse();
    let source_code = fs::read_to_string(args.source_file)
            .expect("The file source file does not exist.");
    let tokens = tokenize(&source_code);
    for token in tokens {
        println!("{:?}", token);
    }
}
