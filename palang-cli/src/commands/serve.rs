use clap::Parser;
use palang_server::{server::ServerArgs, start_server};

#[derive(Debug, Parser)]
pub struct ServeArgs {
    #[arg(long)]
    host: Option<String>,

    #[arg(long)]
    port: Option<u16>,
}

pub fn serve_command(args: &ServeArgs) -> Result<(), String> {
    let host: String = match args.host.clone() {
        Some(host) => host,
        None => "127.0.0.1".to_string(),
    };

    let port = match args.port {
        Some(port) => port,
        None => 8242,
    };

    start_server(&ServerArgs::new(host, port)).map_err(|e| e.to_string())
}
