use crate::server_proxy::{models::status::Status, ServerProxy};


impl ServerProxy {
    pub fn get_status(&self) -> Result<Status, String> {
        Ok(Status::new())
    }
}
