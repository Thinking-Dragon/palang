use crate::server_proxy::{models::profile::Profile, ServerProxy};

impl ServerProxy {
    pub fn get_profiles(&self) -> Vec<Profile> {
        Vec::new()
    }

    pub fn add_profile(&mut self, profile: &Profile) {

    }

    pub fn add_profile_alias(&mut self, name: &String, r#for: &String) {
        
    }
}
