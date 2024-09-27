use crate::api::v1::models::profile::Profile;

use super::storage::{load, load_all, store, NamedData, Storable};

pub struct ProfileService;

impl Storable<Profile> for ProfileService {
    fn get(name: &String) -> Result<Profile, String> {
        load(name, "profiles")
    }

    fn get_all() -> Result<Vec<NamedData<Profile>>, String> {
        load_all("profiles")
    }

    fn set(name: &String, profile: &Profile) -> Result<(), String> {
        store(name, "profiles", profile)
    }
}
