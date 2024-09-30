use std::fs;

use clap::{Parser, Subcommand};
use palang_compiler::{compile_file, compile_package};
use palang_server::api::v1::{
    models::{
        assembly::AssemblySource,
        project::Project
    },
    services::storage::{
        name_data,
        NamedData
    }
};

use crate::{
    assembly_path_util::{
        parse_assembly_path,
        AssemblyPath
    },
    pretty_prints::project::{
        pretty_print_assemblies,
        pretty_print_project,
        pretty_print_projects
    },
    server_proxy::ServerProxy
};

#[derive(Debug, Parser)]
pub struct ProjectsArgs {
    project: Option<String>,

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
    #[command(subcommand)]
    command: Option<AssembliesSubcommand>,
}

#[derive(Debug, Parser)]
enum AssembliesSubcommand {
    New {
        path: String,
    }
}

pub fn projects_command(args: &ProjectsArgs) -> Result<(), String> {
    if let Some(project) = &args.project {
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
                                    AssembliesSubcommand::New { path } => {
                                        new_assembly_command(&project, &path)
                                    },
                                }
                            },
                            None => {
                                let assemblies: Vec<AssemblySource> = ServerProxy::find_server()?
                                    .get_assemblies(&project)?;
                                println!("{}", pretty_print_assemblies(&assemblies));
                                Ok(())
                            },
                        }
                    },
                }
            },
            None => {
                let project: NamedData<Project> = ServerProxy::find_server()?.get_project(project)?;
                println!("{}", pretty_print_project(&project));
                Ok(())
            },
        }
    }
    else {
        let projects: Vec<NamedData<Project>> = ServerProxy::find_server()?.get_projects()?;
        println!("{}", pretty_print_projects(&projects));
        Ok(())
    }
}

fn new_project_command(name: &String) -> Result<(), String> {
    ServerProxy::find_server()?
        .add_project(
            &name_data(
                name.clone(),
                Project::new()
            )
        )
}

fn new_assembly_command(
    project: &String,
    path: &String
) -> Result<(), String> {
    let assembly: AssemblySource = match parse_assembly_path(path)? {
        AssemblyPath::RemoteAssembly(path) => {
            AssemblySource::new_remote(path)
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

            AssemblySource::new_local(code)
        },
    };

    ServerProxy::find_server()?.add_assembly(project, &assembly)
}
