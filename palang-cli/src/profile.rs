use std::{fs, path::PathBuf};

use palang_virtual_machine::llm::model_settings::ModelSettings;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub llm: String,
    pub model: String,
    pub temperature: u32,
    pub max_tokens: u32,
}

pub fn load_profile(file_path: &PathBuf) -> Result<Profile, String> {
    let raw_profile: String = fs::read_to_string(file_path)
                                 .map_err(|e| e.to_string())?;
    let profile: Profile = serde_yaml::from_str(&raw_profile)
                                      .map_err(|e| e.to_string())?;

    Ok(profile)
}

pub fn load_profile_from_directory(file_name: &String, directory: &Option<PathBuf>) -> Result<Profile, String> {
    let file_name_with_extension: String = format!("{}.yaml", file_name);
    match directory {
        Some(directory_path) => {
            let file_path: PathBuf = directory_path.join(file_name_with_extension);
            return load_profile(&file_path);
        },
        None => {
            if let Some(home_directory) = dirs::home_dir() {
                let local_profile_path: PathBuf = home_directory.join(".local/share/palang/profiles")
                                                                .join(file_name_with_extension.clone());
                if local_profile_path.exists() {
                    return load_profile(&local_profile_path);
                }
            }

            let global_profile_path = PathBuf::from("/usr/share/palang/profiles")
                                                       .join(file_name_with_extension);
            if global_profile_path.exists() {
                return load_profile(&global_profile_path);
            }
        },
    }

    return Err(format!("Profile {} was not found.", file_name));
}

impl Profile {
    pub fn get_model_settings(&self) -> ModelSettings {
        ModelSettings {
            model: self.model.clone(),
            temperature: self.temperature,
            max_tokens: self.max_tokens,
        }
    }
}
