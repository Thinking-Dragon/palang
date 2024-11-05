use std::{env, fs::{self, DirEntry}, path::PathBuf};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub trait Storable<T> {
    fn get(name: &String) -> Result<T, String>;
    fn get_all() -> Result<Vec<NamedData<T>>, String>;
    fn set(name: &String, data: &T) -> Result<(), String>;
}

pub fn load<T>(name: &String, collection: &str) -> Result<T, String>
    where T: DeserializeOwned
{
    let file_name_with_extension: String = format!("{}.yaml", name);

    let directory: PathBuf = (
        if let Ok(snap_user_data) = env::var("SNAP_USER_DATA") {
            PathBuf::from(snap_user_data)
        } else {
            dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")).join(".palang")
        }
    ).join(collection);

    let file_path: PathBuf = directory.join(file_name_with_extension);
    load_file(&file_path)
}

pub fn load_all<T>(collection: &str) -> Result<Vec<NamedData<T>>, String>
    where T: DeserializeOwned,
{
    let directory: PathBuf = (
        if let Ok(snap_user_data) = env::var("SNAP_USER_DATA") {
            PathBuf::from(snap_user_data)
        } else {
            dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")).join(".palang")
        }
    ).join(collection);

    if !directory.exists() {
        return Ok(Vec::new());
    }

    let mut results: Vec<NamedData<T>> = Vec::new();

    for entry in fs::read_dir(&directory).map_err(|e| e.to_string())? {
        let entry: DirEntry = entry.map_err(|e| e.to_string())?;
        let path: PathBuf = entry.path();
        let name: String = entry.file_name()
            .into_string()
            .map_err(|e| format!("{:?}", e))?
            .strip_suffix(".yaml")
            .unwrap_or_default()
            .to_string();

        let is_ext_yaml: bool = path.extension().map_or(false, |ext| ext == "yaml");
        if path.is_file() && is_ext_yaml {
            match load_file(&path) {
                Ok(data) => results.push(name_data(name, data)),
                Err(e) => eprintln!("Error loading file {:?}: {}", path, e),
            }
        }
    }

    Ok(results)
}

pub fn store<T>(name: &String, collection: &str, data: &T) -> Result<(), String>
    where T: Serialize
{
    let file_name_with_extension: String = format!("{}.yaml", name);

    let directory: PathBuf = (
        if let Ok(snap_user_data) = env::var("SNAP_USER_DATA") {
            PathBuf::from(snap_user_data)
        } else {
            dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")).join(".palang")
        }
    ).join(collection);

    fs::create_dir_all(&directory)
        .map_err(|e| e.to_string())?;

    let file_path: PathBuf = directory.join(file_name_with_extension);
    store_file(&file_path, data)
}

fn load_file<T>(file_path: &PathBuf) -> Result<T, String>
    where T: DeserializeOwned
{
    let raw_data: String = fs::read_to_string(file_path)
        .map_err(|e| e.to_string())?;

    let data: T = serde_yaml::from_str(&raw_data)
        .map_err(|e| e.to_string())?;

    Ok(data)
}

fn store_file<T>(file_path: &PathBuf, data: &T) -> Result<(), String>
    where T: Serialize
{
    let raw_data: String = serde_yaml::to_string(data)
        .map_err(|e| e.to_string())?;

    fs::write(file_path, raw_data)
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NamedData<T> {
    pub name: String,

    #[serde(flatten)]
    pub data: T,
}

pub fn name_data<T>(name: String, data: T) -> NamedData<T> {
    NamedData { name, data }
}
