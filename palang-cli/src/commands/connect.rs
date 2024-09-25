use clap::Parser;

use crate::server_proxy::ServerProxy;

#[derive(Debug, Parser)]
pub struct ConnectArgs {
    #[arg(short, long)]
    host: String,

    #[arg(short, long)]
    port: u16,
}

pub fn connect_command(args: &ConnectArgs) -> Result<(), String> {
    ServerProxy::connect(&args.host, &args.port)
}
