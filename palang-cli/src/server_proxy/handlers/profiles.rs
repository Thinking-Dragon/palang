use palang_server::api::v1::{models::profile::{Profile, ProfileAlias}, services::storage::NamedData};

use crate::server_proxy::ServerProxy;

impl ServerProxy {
    pub fn get_profiles(&self) -> Result<Vec<NamedData<Profile>>, String> {
        self.get("profiles")
    }

    pub fn add_profile(&mut self, profile: &NamedData<Profile>) -> Result<(), String> {
        self.post_only("profiles", &profile)
    }

    pub fn add_profile_alias(&mut self, alias: &ProfileAlias) -> Result<(), String> {
        self.post("profiles/alias", &alias)
    }
}
