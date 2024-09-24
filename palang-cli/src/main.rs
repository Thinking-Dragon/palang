mod profile;
mod dialog_utils;
mod commands;
mod server_proxy;

use clap::{Parser, Subcommand};
use commands::{
    compile::{
        compile_command,
        CompileArgs
    },
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
    Compile(CompileArgs),
    Run(RunArgs),
    Serve(ServeArgs),
    Status,
    Projects(ProjectsArgs),
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
