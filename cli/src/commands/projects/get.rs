use clap::{
    Parser,
    Subcommand
};

use palang_core::{
    language::assembly_source::WrappedAssembly,
    project::WrappedProject,
    storage::NamedData
};

use crate::{
    pretty_prints::project::{
        pretty_print_assembly_sources,
        pretty_print_project
    },
    server_proxy::ServerProxy
};

use super::assemblies::{
    assemblies_command,
    AssembliesArgs
};

#[derive(Debug, Parser)]
pub struct ProjectArgs {
    name: String,

    #[command(subcommand)]
    command: Option<ProjectCommand>,
}

#[derive(Debug, Subcommand)]
enum ProjectCommand {
    Assemblies,
    Assembly(AssembliesArgs),
}

pub fn project_command(args: &ProjectArgs) -> Result<(), String> {
    if let Some(command) = &args.command {
        match command {
            ProjectCommand::Assemblies => {
                let assemblies: Vec<WrappedAssembly> = ServerProxy::find_server()?
                    .get_assemblies(&args.name)?;
    
                println!("{}", pretty_print_assembly_sources(&assemblies)?);
                Ok(())
            },
            ProjectCommand::Assembly(asm_args) => {
                assemblies_command(&args.name, asm_args)
            },
        }
    }
    else {
        let project: NamedData<WrappedProject> = ServerProxy::find_server()?.get_project(&args.name)?;
        println!("{}", pretty_print_project(&project)?);
        Ok(())
    }
}
