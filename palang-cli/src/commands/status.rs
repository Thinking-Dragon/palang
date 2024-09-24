use crate::server_proxy::ServerProxy;

pub fn status_command() -> Result<(), String> {
    let proxy: ServerProxy = ServerProxy::find_server()?;
    let status = proxy.get_status()?;

    println!("{}", status.to_string());
    Ok(())
}
