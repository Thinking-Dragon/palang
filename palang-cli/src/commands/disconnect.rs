use crate::server_proxy::ServerProxy;

pub fn disconnect_command() -> Result<(), String> {
    ServerProxy::disconnect()
}
