use crate::server_proxy::{models::profile::{Profile, ProfileAlias}, ServerProxy};

impl ServerProxy {
    pub fn get_profiles(&self) -> Result<Vec<Profile>, String> {
        self.get("profiles")
    }

    pub fn add_profile(&mut self, profile: &Profile) -> Result<(), String> {
        self.post("profiles", &profile)
    }

    pub fn add_profile_alias(&mut self, alias: &ProfileAlias) -> Result<(), String> {
        self.post("profiles/alias", &alias)
    }
}
