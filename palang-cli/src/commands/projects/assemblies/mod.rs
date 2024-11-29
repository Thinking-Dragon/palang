pub mod add;

use add::add_assembly_command;

use clap::{
    Parser,
    Subcommand
};

#[derive(Debug, Parser)]
pub struct AssembliesArgs {
    #[command(subcommand)]
    command: AssembliesCommand,
}

#[derive(Debug, Subcommand)]
enum AssembliesCommand {
    Add {
        path: String
    },
}

pub fn assemblies_command(project: &String, args: &AssembliesArgs) -> Result<(), String> {
    match &args.command {
        AssembliesCommand::Add { path } => {
            add_assembly_command(project, &path)
        }
    }
}
