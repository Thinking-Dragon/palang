mod profile;
mod dialog_utils;
mod server_proxy;

use std::{env, fs, path::{Path, PathBuf}};

use clap::{Parser, Subcommand};
use dialog_utils::ask;
use palang_compiler::{compile_file, compile_package};
use palang_server::{server::ServerArgs, start_server};
use palang_virtual_machine::{assembly::{assembly::Assembly, loader::load_assembly}, boot_machine, choose_llm, load_assembly_file, virtualization::virtual_machine::VirtualMachine};
use profile::{import_profile, load_profile, load_profile_from_directory, Profile};
use server_proxy::{models::project::Project, ServerProxy};
use tokio::runtime::Runtime;

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

#[derive(Debug, Parser)]
struct CompileArgs {
    #[arg(short, long)]
    source: Option<PathBuf>,

    #[arg(short, long)]
    package: Option<PathBuf>,

    #[arg(short, long)]
    target: Option<PathBuf>,
}

#[derive(Debug, Parser)]
struct RunArgs {
    #[arg(value_name = "ASSEMBLY FILE")]
    assembly_file: PathBuf,

    #[arg(short, long)]
    task: String,

    #[arg(short, long, num_args = 1.., value_delimiter = ',')]
    args: Vec<String>,

    #[arg(short, long)]
    profile: String,

    #[arg(long)]
    profiles_directory: Option<PathBuf>,
}

#[derive(Debug, Parser)]
struct ServeArgs {
    #[arg(short, long)]
    host: Option<String>,

    #[arg(short, long)]
    port: Option<u16>,
}

#[derive(Debug, Parser)]
struct ProjectsArgs {
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

#[derive(Debug, Parser)]
struct ProfilesArgs {
    #[command(subcommand)]
    command: ProfilesCommand,
}

#[derive(Debug, Subcommand)]
enum ProfilesCommand {
    New(NewProfileArgs),
    NewAlias(NewAliasArgs),
}

#[derive(Debug, Parser)]
struct NewProfileArgs {
    name: String,

    #[arg(long)]
    from: Option<PathBuf>,
}

#[derive(Debug, Parser)]
struct NewAliasArgs {
    name: String,

    #[arg(long)]
    r#for: String,
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

fn compile_command(args: &CompileArgs) -> Result<(), String> {
    if let Some(target) = &args.target {
        if let Some(source) = &args.source {
            return compile_file_to_target(&source, &target);
        }
        else if let Some(package) = &args.package {
            return compile_package_to_target(&package, &target);
        }
        else {
            if let Ok(package) = env::current_dir().map_err(|e| e.to_string()) {
                return compile_package_to_target(&package, &target);
            }
            else {
                return Err("Working directory not found, please specify a source or package directory".to_string());
            }
        }
    }
    else {
        return Err("No compilation target specified".to_string());
    }
}

fn compile_file_to_target(source_path: &Path, target_path: &Path) -> Result<(), String> {
    println!("Compiling {:?} to {:?}", source_path, target_path);

    let source_code: String = fs::read_to_string(source_path).map_err(|e| e.to_string())?;
    let assembly_code: String = compile_file(&source_code)?;

    fs::write(&target_path, assembly_code).map_err(|e| e.to_string())?;

    Ok(())
}

fn compile_package_to_target(package_root: &Path, target_path: &Path) -> Result<(), String> {
    println!("Compiling package {:?} to {:?}", package_root, target_path);

    let assembly_code: String = compile_package(&package_root)?;
    fs::write(&target_path, assembly_code).map_err(|e| e.to_string())?;

    Ok(())
}

fn run_command(args: &RunArgs) -> Result<(), String> {
    match load_profile_from_directory(
        &args.profile,
        &args.profiles_directory,
    ) {
        Ok(profile) => {
            match choose_llm(&profile.llm) {
                Ok(llm) => {
                    match get_assembly(&args.assembly_file) {
                        Ok(asm) => {
                            let mut vm: VirtualMachine = boot_machine(&llm);
                            vm.load_assembly(&asm);

                            let runtime: Runtime = tokio::runtime::Runtime::new().unwrap();
                            let result: Result<String, String> = runtime.block_on(async {
                                vm.execute(
                                    &args.task,
                                    &args.args,
                                    &profile.get_model_settings()
                                ).await.await
                            });
                            match result {
                                Ok(output) => {
                                    println!("{}", output);
                                    Ok(())
                                },
                                Err(e) => {
                                    return Err(format!("Could not execute program ({})", e));
                                },
                            }
                        },
                        Err(e) => {
                            return Err(
                                format!(
                                    "Could not find assembly file {:?} ({})",
                                    args.assembly_file,
                                    e
                                )
                            );
                        }
                    }
                },
                Err(e) => {
                    return Err(format!("Specified large language model \"{}\" not found ({})", profile.llm, e));
                },
            }
        },
        Err(e) => Err(format!("{}", e)),
    }
}

fn serve_command(args: &ServeArgs) -> Result<(), String> {
    let host: String = match args.host.clone() {
        Some(host) => host,
        None => "127.0.0.1".to_string(),
    };

    let port = match args.port {
        Some(port) => port,
        None => 8242,
    };

    start_server(&ServerArgs::new(host, port)).map_err(|e| e.to_string())
}

fn status_command() -> Result<(), String> {
    let proxy: ServerProxy = get_server_proxy()?;
    let status = proxy.get_status()?;

    println!("{}", status.to_string());
    Ok(())
}

fn projects_command(args: &ProjectsArgs) -> Result<(), String> {
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
    let mut proxy: ServerProxy = get_server_proxy()?;
    
    proxy.add_project(&Project::new());

    Ok(())
}

fn new_assembly_command(project: &String, assembly: &String, path: &String) -> Result<(), String> {
    let mut proxy: ServerProxy = get_server_proxy()?;

    proxy.add_assembly(&crate::server_proxy::models::assembly::Assembly::new());

    Ok(())
}

fn profiles_command(args: &ProfilesArgs) -> Result<(), String> {
    match &args.command {
        ProfilesCommand::New(new_args) => {
            new_profile_command(&new_args.name, &new_args.from)
        },
        ProfilesCommand::NewAlias(alias_args) => {
            new_profile_alias_command(&alias_args.name, &alias_args.r#for)
        },
    }
}

fn new_profile_command(name: &String, from: &Option<PathBuf>) -> Result<(), String> {
    if let Some(from) = from {
        let profile: Profile = load_profile(from)?;
        import_profile(name, &profile)
    }
    else {
        new_profile_dialog(name)
    }
    // TODO: move to server if connected
}

fn new_profile_dialog(name: &String) -> Result<(), String> {
    let llm:             String = ask("Which LLM provider to use")?;
    let model:           String = ask("Which model you want to use")?;
    let mut temperature: String = ask("Temperature [0.7]")?;
    let mut max_tokens:  String = ask("Maximum number of tokens [1024]")?;

    if temperature.is_empty() {
        temperature = "0.7".to_string();
    }
    let temperature_float: f32 = temperature.parse::<f32>().map_err(|e| e.to_string())?;

    if max_tokens.is_empty() {
        max_tokens = "1024".to_string();
    }
    let max_tokens_int: u32 = max_tokens.parse::<u32>().map_err(|e| e.to_string())?;

    let profile: Profile = Profile::new(
        llm,
        model,
        temperature_float,
        max_tokens_int,
    );

    import_profile(name, &profile)
    // TODO: move to server if connected
}

fn new_profile_alias_command(name: &String, r#for: &String) -> Result<(), String> {
    let mut proxy: ServerProxy = get_server_proxy()?;
    
    proxy.add_profile_alias(name, r#for);
    
    Ok(())
}

fn get_assembly(file_path: &PathBuf) -> Result<Assembly, String> {
    let extension = file_path.extension().and_then(|ext| ext.to_str()).unwrap_or("");

    match extension {
        "palasm" => load_assembly_file(file_path),
        "palang" => {
            let source_code: String = fs::read_to_string(file_path).map_err(|e| e.to_string())?;
            let assembly_code: String = compile_file(&source_code)?;
            load_assembly(&assembly_code)
        },
        _ => Err(format!("Unsupported file extension: {}", extension)),
    }
}

fn get_server_proxy() -> Result<ServerProxy, String> {
    Ok(ServerProxy::from(&"127.0.0.1".to_string(), &8242))
}
