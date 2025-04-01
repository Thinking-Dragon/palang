use std::{
    fs,
    path::PathBuf
};

use palang_compiler::compile_file;

use palang_virtual_machine::{
    assembly::{
        assembly::Assembly,
        loader::load_assembly
    },
    load_assembly_file
};

pub fn get_assembly_from_file(file_path: &PathBuf) -> Result<Assembly, String> {
    let extension = file_path.extension().and_then(|ext| ext.to_str()).unwrap_or("");

    match extension {
        "palasm" => load_assembly_file(file_path),
        "palang" => {
            let source_code: String = fs::read_to_string(file_path).map_err(|e| e.to_string())?;
            let assembly_code: String = compile_file(&source_code)?;
            load_assembly(&assembly_code)
        },
        _ => Err(format!("Unsupported file extension: {}", extension)),
    }
}
