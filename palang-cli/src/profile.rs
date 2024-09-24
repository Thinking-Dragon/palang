use std::{env, fs, path::PathBuf};

use palang_virtual_machine::llm::model_settings::ModelSettings;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub llm: String,
    pub model: String,
    pub temperature: f32,
    pub max_tokens: u32,
}

impl Profile {
    pub fn new(
        llm: String,
        model: String,
        temperature: f32,
        max_tokens: u32,
    ) -> Self {
        Profile { llm, model, temperature, max_tokens }
    }

    pub fn get_model_settings(&self) -> ModelSettings {
        ModelSettings {
            model: self.model.clone(),
            temperature: self.temperature,
            max_tokens: self.max_tokens,
        }
    }
}

pub fn load_profile(file_path: &PathBuf) -> Result<Profile, String> {
    let raw_profile: String = fs::read_to_string(file_path)
                                 .map_err(|e| e.to_string())?;
    let profile: Profile = serde_yaml::from_str(&raw_profile)
                                      .map_err(|e| e.to_string())?;

    Ok(profile)
}

pub fn write_profile(file_path: &PathBuf, profile: &Profile) -> Result<(), String> {
    let raw_profile: String = serde_yaml::to_string(profile)
                                         .map_err(|e| e.to_string())?;

    fs::write(file_path, raw_profile).map_err(|e| e.to_string())?;

    Ok(())
}

pub fn load_profile_from_directory(name: &String, directory: &Option<PathBuf>) -> Result<Profile, String> {
    let file_name_with_extension = format!("{}.yaml", name);

    let base_directory = (if let Some(dir) = directory {
        dir.clone()
    } else if let Ok(snap_user_data) = env::var("SNAP_USER_DATA") {
        PathBuf::from(snap_user_data)
    } else {
        dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")).join(".palang")
    }).join("profiles");

    let file_path = base_directory.join(file_name_with_extension);
    load_profile(&file_path)
}

pub fn import_profile(name: &String, profile: &Profile) -> Result<(), String> {
    let file_name_with_extension = format!("{}.yaml", name);

    let base_directory = (if let Ok(snap_user_data) = env::var("SNAP_USER_DATA") {
        PathBuf::from(snap_user_data)
    } else {
        dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")).join(".palang")
    }).join("profiles");

    fs::create_dir_all(&base_directory).map_err(|e| e.to_string())?;

    let file_path = base_directory.join(file_name_with_extension);
    write_profile(&file_path, profile)
}
