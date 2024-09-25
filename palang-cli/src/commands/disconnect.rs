use crate::server_proxy::ServerProxy;

pub fn disconnect_command() -> Result<(), String> {
    if !ServerProxy::is_connected() {
        return Err("Not connected to any server".to_string());
    }

    let previous_server: ServerProxy = ServerProxy::find_server()?;
    ServerProxy::disconnect()?;
    println!("Disconnected from {}:{}", previous_server.host, previous_server.port);
    Ok(())
}
