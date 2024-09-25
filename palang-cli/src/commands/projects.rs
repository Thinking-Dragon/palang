use std::fs;

use clap::{Parser, Subcommand};
use palang_compiler::{compile_file, compile_package};
use tabled::Table;

use crate::{
    assembly_path_util::{
        parse_assembly_path,
        AssemblyPath
    },
    server_proxy::{
        models::{
            assembly::Assembly,
            project::Project
        },
        ServerProxy
    }
};

#[derive(Debug, Parser)]
pub struct ProjectsArgs {
    #[command(subcommand)]
    command: Option<ProjectsCommand>,
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
    command: Option<AssembliesSubcommand>,
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
        Some(command) => {
            match command {
                ProjectsCommand::New { name } => {
                    new_project_command(&name)
                },
                ProjectsCommand::Assemblies(assemblies_args) => {
                    match &assemblies_args.command {
                        Some(command) => {
                            match command {
                                AssembliesSubcommand::New { assembly, path } => {
                                    new_assembly_command(&assemblies_args.project, &assembly, &path)
                                },
                            }
                        },
                        None => {
                            let assemblies: Vec<Assembly> = ServerProxy::find_server()?
                                .get_assemblies(&assemblies_args.project)?;
                            println!("Assemblies:\n{}", Table::new(assemblies).to_string());
                            Ok(())
                        },
                    }
                },
            }
        },
        None => {
            let projects: Vec<Project> = ServerProxy::find_server()?.get_projects()?;
            println!("Projects:\n{}", Table::new(projects).to_string());
            Ok(())
        },
    }
}

fn new_project_command(name: &String) -> Result<(), String> {
    ServerProxy::find_server()?.add_project(&Project::new(name.clone()))
}

fn new_assembly_command(
    project: &String,
    assembly: &String,
    path: &String
) -> Result<(), String> {
    let assembly: Assembly = match parse_assembly_path(path)? {
        AssemblyPath::RemoteAssembly(path) => {
            Assembly::new_remote(project.clone(), assembly.clone(), path)
        },
        AssemblyPath::LocalAssembly(path) => {
            let code = match path.extension().and_then(|ext| ext.to_str()) {
                Some(extension) => {
                    match extension {
                        "palasm" => {
                            fs::read_to_string(path).map_err(|e| e.to_string())
                        },
                        "palang" => {
                            let source_code: String = fs::read_to_string(path).map_err(|e| e.to_string())?;
                            compile_file(&source_code)
                        },
                        _ => Err(format!("Unsupported file extension: {}", extension)),
                    }
                },
                None => {
                    compile_package(&path)
                },
            }?;

            Assembly::new_local(project.clone(), assembly.clone(), code)
        },
    };

    ServerProxy::find_server()?.add_assembly(project, &assembly)
}
