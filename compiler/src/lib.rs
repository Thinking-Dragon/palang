use std::{fs, path::{Path, PathBuf}};

use package::{load_package_description, Package};
use tokenize::{tokenizer::tokenize, tokens::Token};
use parse::{ast_node::ASTNode, parser::parse};
use analyze::semantic_analyzer::analyze_semantics;
use generate::code_generator::generate_palassembly;
use walkdir::WalkDir;

pub mod tokenize;
pub mod parse;
pub mod analyze;
pub mod generate;
pub mod package;

pub fn compile_package(root: &Path) -> Result<String, String> {
    let package: Package = load_package_description(root)?;
    let source_files: Vec<PathBuf> = WalkDir::new(root)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry.path()
                 .extension()
                 .map_or(false, |extension| extension == "palang")
        })
        .map(|entry| entry.path().to_path_buf())
        .collect();

    let mut package_assembly = format!("PACKAGE {}\n", package.name);
    package_assembly.push_str(&format!("DESCRIPTION\nSTART\n{}\nEND\n", package.description));
    package_assembly.push_str(&format!("VERSION {}\n", package.version));
    for source_file in source_files {
        let source_code: String = fs::read_to_string(source_file).map_err(|e| e.to_string())?;
        let assembly: String = compile_file(&source_code)?;
        package_assembly.push_str(&assembly);
    }

    Ok(package_assembly)
}

pub fn compile_file(source_code: &String) -> Result<String, String> {
    let tokens: Vec<Token> = tokenize(source_code);
    let ast: ASTNode = parse(tokens).map_err(|e| e)?;
    analyze_semantics(&ast)?;
    generate_palassembly(&ast)
}
