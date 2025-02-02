mod api;
mod server;

use clap::Parser;
use server::start_server;

#[derive(Debug, Parser)]
struct Arguments {
    #[arg(long)]
    host: Option<String>,

    #[arg(long)]
    port: Option<u16>,
}

fn main() {
    let arguments: Arguments = Arguments::parse();

    let host: String = match arguments.host {
        Some(host) => host,
        None => "127.0.0.1".to_string(),
    };

    let port: u16 = match arguments.port {
        Some(port) => port,
        None => 8242,
    };

    if let Err(e) = start_server(host, port) {
        eprintln!("{}", e);
    }
}
