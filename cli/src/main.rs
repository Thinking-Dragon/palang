mod assembly_utils;
mod dialog_utils;
mod assembly_path_util;
mod pretty_prints;

mod commands;
mod server_proxy;

use clap::{
    Parser,
    Subcommand
};

use commands::{
    compile::{
        compile_command,
        CompileArgs
    }, connect::{
        connect_command,
        ConnectArgs
    }, describe::{describe_command, DescribeArgs}, disconnect::disconnect_command, profiles::{
        profiles_command,
        ProfilesArgs
    }, projects::{
        get::{
            project_command,
            ProjectArgs
        },
        get_all::projects_command,
        new::{
            new_project_command,
            NewProjectArgs
        }
    }, run::{
        run_command,
        RunArgs
    }, start::{
        start_command,
        StartCommandArgs
    }
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

    #[command(about = "Describe a program's contents")]
    Describe(DescribeArgs),

    #[command(about = "Start a Palang server or registry")]
    Start(StartCommandArgs),

    #[command(about = "Connect to a Palang server")]
    Connect(ConnectArgs),

    #[command(about = "Disconnect from Palang server")]
    Disconnect,

    #[command(about = "List all projects")]
    Projects,

    #[command(about = "Manage a project")]
    Project(ProjectArgs),

    #[command(about = "Create a new project")]
    NewProject(NewProjectArgs),

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
        Command::Describe(args) => {
            describe_command(&args)
        },
        Command::Start(args) => {
            start_command(&args)
        },
        Command::Connect(args) => {
            connect_command(&args)
        },
        Command::Disconnect => {
            disconnect_command()
        },
        Command::Projects => {
            projects_command()
        },
        Command::NewProject(args) => {
            new_project_command(&args)
        },
        Command::Project(args) => {
            project_command(&args)
        },
        Command::Profiles(args) => {
            profiles_command(&args)
        },
    }
}
