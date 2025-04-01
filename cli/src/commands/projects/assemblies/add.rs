use std::fs;

use palang_compiler::{
    compile_file,
    compile_package
};

use palang_core::language::assembly_source::AssemblySource;

use crate::{
    assembly_path_util::{
        parse_assembly_path,
        AssemblyPath
    },
    server_proxy::ServerProxy
};

pub fn add_assembly_command(project: &String, path: &String) -> Result<(), String> {
    let assembly: AssemblySource = match parse_assembly_path(path)? {
        AssemblyPath::RemoteAssembly(path) => {
            AssemblySource::new_remote(path)
        },
        AssemblyPath::LocalAssembly(path) => {
            let code = match path.extension().and_then(|ext| ext.to_str()) {
                Some(extension) => {
                    match extension {
                        "palasm" => {
                            fs::read_to_string(path).map_err(|e| e.to_string())
                        },
                        "palang" => {
                            let source_code: String = fs::read_to_string(path).map_err(|e| e.to_string())?;
                            compile_file(&source_code)
                        },
                        _ => Err(format!("Unsupported file extension: {}", extension)),
                    }
                },
                None => {
                    compile_package(&path)
                },
            }?;

            AssemblySource::new_local(code)
        },
    };

    ServerProxy::find_server()?.add_assembly(project, &assembly)
}
