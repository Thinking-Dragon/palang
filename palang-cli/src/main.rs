mod dialog_utils;
mod assembly_path_util;
mod commands;
mod server_proxy;

use clap::{Parser, Subcommand};
use commands::{
    compile::{
        compile_command,
        CompileArgs
    },
    connect::{
        connect_command,
        ConnectArgs
    },
    disconnect::disconnect_command,
    profiles::{
        profiles_command,
        ProfilesArgs
    },
    projects::{
        projects_command,
        ProjectsArgs
    },
    run::{
        run_command,
        RunArgs
    },
    serve::{
        serve_command,
        ServeArgs
    },
    status::status_command
};

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command
}

#[derive(Debug, Subcommand)]
enum Command {
    #[command(about = "Compile a source file")]
    Compile(CompileArgs),

    #[command(about = "Run a compiled program")]
    Run(RunArgs),

    #[command(about = "Start a Palang server")]
    Serve(ServeArgs),

    #[command(about = "Connect to a Palang server")]
    Connect(ConnectArgs),

    #[command(about = "Disconnect from Palang server")]
    Disconnect,

    #[command(about = "Print current status of server")]
    Status,

    #[command(about = "Manage projects")]
    Projects(ProjectsArgs),

    #[command(about = "Manage profiles")]
    Profiles(ProfilesArgs),
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    source_file: String,

    #[arg(short, long)]
    target_file: String,
}

fn main() {
    if let Err(e) = execute_command() {
        eprintln!("{}", e);
    }
}

fn execute_command() -> Result<(), String> {
    match Cli::parse().command {
        Command::Compile(args) => {
            compile_command(&args)
        },
        Command::Run(args) => {
            run_command(&args)
        },
        Command::Serve(args) => {
            serve_command(&args)
        },
        Command::Connect(args) => {
            connect_command(&args)
        },
        Command::Disconnect => {
            disconnect_command()
        },
        Command::Status => {
            status_command()
        },
        Command::Projects(args) => {
            projects_command(&args)
        },
        Command::Profiles(args) => {
            profiles_command(&args)
        },
    }
}
