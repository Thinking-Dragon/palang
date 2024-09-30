use palang_server::api::v1::models::status::Status;

use crate::server_proxy::ServerProxy;

impl ServerProxy {
    pub fn get_status(&self) -> Result<Status, String> {
        self.get("status")
    }
}
