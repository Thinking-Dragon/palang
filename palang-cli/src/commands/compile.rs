use std::{env, fs, path::{Path, PathBuf}};

use clap::{arg, Parser};
use palang_compiler::{compile_file, compile_package};

#[derive(Debug, Parser)]
pub struct CompileArgs {
    #[arg(short, long)]
    source: Option<PathBuf>,

    #[arg(short, long)]
    package: Option<PathBuf>,

    #[arg(short, long)]
    target: Option<PathBuf>,
}

pub fn compile_command(args: &CompileArgs) -> Result<(), String> {
    if let Some(target) = &args.target {
        if let Some(source) = &args.source {
            return compile_file_to_target(&source, &target);
        }
        else if let Some(package) = &args.package {
            return compile_package_to_target(&package, &target);
        }
        else {
            if let Ok(package) = env::current_dir().map_err(|e| e.to_string()) {
                return compile_package_to_target(&package, &target);
            }
            else {
                return Err("Working directory not found, please specify a source or package directory".to_string());
            }
        }
    }
    else {
        return Err("No compilation target specified".to_string());
    }
}

fn compile_file_to_target(source_path: &Path, target_path: &Path) -> Result<(), String> {
    println!("Compiling {:?} to {:?}", source_path, target_path);

    let source_code: String = fs::read_to_string(source_path).map_err(|e| e.to_string())?;
    let assembly_code: String = compile_file(&source_code)?;

    fs::write(&target_path, assembly_code).map_err(|e| e.to_string())?;

    Ok(())
}

fn compile_package_to_target(package_root: &Path, target_path: &Path) -> Result<(), String> {
    println!("Compiling package {:?} to {:?}", package_root, target_path);

    let assembly_code: String = compile_package(&package_root)?;
    fs::write(&target_path, assembly_code).map_err(|e| e.to_string())?;

    Ok(())
}
