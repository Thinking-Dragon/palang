use std::process::Command;

use clap::Parser;

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

    let port: u16 = match args.port {
        Some(port) => port,
        None => 8242,
    };

    let server_process = Command::new("palang-server")
        .args(&["--host", &host, "--port", &port.to_string()])
        .spawn();

    if let Ok(mut server_process) = server_process {
        server_process.wait().map_err(|e| format!("Server crashed: {}", e.to_string()))?;
        Ok(())
    }
    else {
        Err("Could not start server, make sure the `palang-server` package is installed locally and accessible through the command line.".to_string())
    }
}
