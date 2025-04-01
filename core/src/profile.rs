use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, Serialize, Deserialize, Tabled)]
pub struct Profile {
    pub llm: String,
    pub model: String,
    pub temperature: f32,
    pub max_tokens: u32,
    #[tabled(display_with = "display_option_string")]
    pub api_key: Option<String>,
}

impl Profile {
    pub fn new(
        llm: String,
        model: String,
        temperature: f32,
        max_tokens: u32,
        api_key: Option<String>,
    ) -> Self {
        Profile { llm, model, temperature, max_tokens, api_key }
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

pub fn load_profile_from_directory(name: &String, _directory: &Option<PathBuf>) -> Result<Profile, String> {
    let file_name_with_extension = format!("{}.yaml", name);

    let base_directory: PathBuf = dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")).join(".palang").join("profiles");
    // let base_directory: PathBuf = {
    //     let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    //     let snap_palang_dir = home_dir.join("snap").join("palang");

    //     if snap_palang_dir.exists() && snap_palang_dir.is_dir() {
    //         snap_palang_dir.join("common")
    //     } else {
    //         home_dir.join(".palang")
    //     }
    // }.join("profiles");

    let file_path = base_directory.join(file_name_with_extension);
    load_profile(&file_path)
}

pub fn import_profile(name: &String, profile: &Profile) -> Result<(), String> {
    let file_name_with_extension = format!("{}.yaml", name);

    let base_directory: PathBuf = dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")).join(".palang").join("profiles");
    // let base_directory: PathBuf = {
    //     let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    //     let snap_palang_dir = home_dir.join("snap").join("palang");

    //     if snap_palang_dir.exists() && snap_palang_dir.is_dir() {
    //         snap_palang_dir.join("common")
    //     } else {
    //         home_dir.join(".palang")
    //     }
    // }.join("profiles");

    fs::create_dir_all(&base_directory).map_err(|e| e.to_string())?;

    let file_path = base_directory.join(file_name_with_extension);
    write_profile(&file_path, profile)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProfileAlias {
    pub name: String,
    pub r#for: String,
}

impl ProfileAlias {
    pub fn new(name: String, r#for: String) -> Self {
        ProfileAlias { name, r#for }
    }
}

fn display_option_string(o: &Option<String>) -> String {
    o.as_deref().unwrap_or("N/A").to_string()
}
