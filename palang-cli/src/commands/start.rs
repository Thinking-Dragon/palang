use std::process::Command;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct StartCommandArgs {
    #[command(subcommand)]
    command: StartCommand,
}

#[derive(Debug, Subcommand)]
pub enum StartCommand {
    #[command(about = "")]
    Server(StartArgs),

    #[command(about = "")]
    Registry(StartArgs),
}

#[derive(Debug, Parser)]
pub struct StartArgs {
    #[arg(long)]
    host: Option<String>,

    #[arg(long)]
    port: Option<u16>,
}

pub fn start_command(args: &StartCommandArgs) -> Result<(), String> {
    match &args.command {
        StartCommand::Server(args) => start_server(args),
        StartCommand::Registry(args) => start_registry(args),
    }
}

fn start_server(args: &StartArgs) -> Result<(), String> {
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

fn start_registry(args: &StartArgs) -> Result<(), String> {
    let host: String = match args.host.clone() {
        Some(host) => host,
        None => "127.0.0.1".to_string(),
    };

    let port: u16 = match args.port {
        Some(port) => port,
        None => 8232,
    };

    let registry_process = Command::new("palang-registry")
        .args(&["--host", &host, "--port", &port.to_string()])
        .spawn();

    if let Ok(mut server_process) = registry_process {
        server_process.wait().map_err(|e| format!("Registry crashed: {}", e.to_string()))?;
        Ok(())
    }
    else {
        Err("Could not start registry, make sure the `palang-registry` package is installed locally and accessible through the command line.".to_string())
    }
}
