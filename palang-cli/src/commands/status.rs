use crate::server_proxy::ServerProxy;

pub fn status_command() -> Result<(), String> {
    let status = ServerProxy::find_server()?.get_status()?;
    println!("{}", status.to_pretty_format());

    Ok(())
}
