pub mod models;
pub mod handlers;

pub struct ServerProxy {

}

impl ServerProxy {
    pub fn from(host: &String, port: &u16) -> Self {
        ServerProxy {}
    }

    pub fn find_server() -> Result<ServerProxy, String> {
        Ok(ServerProxy::from(&"127.0.0.1".to_string(), &8242))
    }
}
