use clap::{Parser, Subcommand};

use crate::server_proxy::{models::project::Project, ServerProxy};

#[derive(Debug, Parser)]
pub struct ProjectsArgs {
    #[command(subcommand)]
    command: ProjectsCommand,
}

#[derive(Debug, Subcommand)]
enum ProjectsCommand {
    New { name: String },
    Assemblies(AssembliesArgs),
}

#[derive(Debug, Parser)]
struct AssembliesArgs {
    project: String,

    #[command(subcommand)]
    command: AssembliesSubcommand,
}

#[derive(Debug, Parser)]
enum AssembliesSubcommand {
    New {
        assembly: String,

        #[arg(long)]
        path: String,
    }
}

pub fn projects_command(args: &ProjectsArgs) -> Result<(), String> {
    match &args.command {
        ProjectsCommand::New { name } => {
            new_project_command(&name)
        },
        ProjectsCommand::Assemblies(assemblies_args) => {
            match &assemblies_args.command {
                AssembliesSubcommand::New { assembly, path } => {
                    new_assembly_command(&assemblies_args.project, &assembly, &path)
                }
            }
        },
    }
}

fn new_project_command(name: &String) -> Result<(), String> {
    let mut proxy: ServerProxy = ServerProxy::find_server()?;
    
    proxy.add_project(&Project::new());

    Ok(())
}

fn new_assembly_command(project: &String, assembly: &String, path: &String) -> Result<(), String> {
    let mut proxy: ServerProxy = ServerProxy::find_server()?;

    proxy.add_assembly(&crate::server_proxy::models::assembly::Assembly::new());

    Ok(())
}
