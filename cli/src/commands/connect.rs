use clap::Parser;

use crate::server_proxy::ServerProxy;

#[derive(Debug, Parser)]
pub struct ConnectArgs {
    #[arg(long)]
    host: String,

    #[arg(long)]
    port: u16,
}

pub fn connect_command(args: &ConnectArgs) -> Result<(), String> {
    let was_connected_to_another_server: bool = ServerProxy::is_connected();
    let previous_server: Option<ServerProxy> = match was_connected_to_another_server {
        true => Some(ServerProxy::find_server()?),
        false => None,
    };

    ServerProxy::connect(&args.host, &args.port)?;
    if was_connected_to_another_server {
        let previous_server = previous_server.unwrap();
        println!("Disconnected from {}:{}", previous_server.host, previous_server.port);
    }
    println!("Connected to {}:{}", &args.host, &args.port);

    Ok(())
}
