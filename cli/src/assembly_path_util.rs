use std::path::PathBuf;

pub enum AssemblyPath {
    RemoteAssembly(String),
    LocalAssembly(PathBuf),
}

pub fn parse_assembly_path(path: &String) -> Result<AssemblyPath, String> {
    if path.starts_with("remote:") {
        let path: String = path[7..].to_string();
        Ok(AssemblyPath::RemoteAssembly(path))
    }
    else if path.starts_with("local:") {
        let path: PathBuf = PathBuf::from(path[6..].to_string());
        Ok(AssemblyPath::LocalAssembly(path))
    }
    else {
        Err("Assembly path must start with 'remote:' or 'local:'".to_string())
    }
}
