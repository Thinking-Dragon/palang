use std::{fs, path::{Path, PathBuf}};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub description: String,
    pub version: String,
}

pub fn load_package_description(root: &Path) -> Result<Package, String> {
    let module_path: PathBuf = root.join("package.yaml");
    let raw_module_description: String = fs::read_to_string(module_path)
                                            .map_err(|e| e.to_string())?;
    let module_description: Package = serde_yaml::from_str(&raw_module_description)
                                                           .map_err(|e| e.to_string())?;

    Ok(module_description)
}
