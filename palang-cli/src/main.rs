pub mod profile;
pub mod dialog_utils;

use std::{env, fs, path::{Path, PathBuf}};

use clap::{Parser, Subcommand};
use dialog_utils::ask;
use palang_compiler::{compile_file, compile_package};
use palang_virtual_machine::{boot_machine, choose_llm, load_assembly_file, virtualization::virtual_machine::VirtualMachine};
use profile::{import_profile, load_profile_from_directory, Profile};
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
    New(NewArgs),
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
struct NewArgs {
    #[arg(value_name = "What to create? [profile]")]
    thing: String,
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
    match Cli::parse().command {
        Command::Compile(args) => {
            compile_command(&args);
        },
        Command::Run(args) => {
            run_command(&args);
        },
        Command::New(args) => {
            new_command(&args);
        }
    }
}

fn compile_command(args: &CompileArgs) {
    if let Some(target) = &args.target {
        if let Some(source) = &args.source {
            if let Err(e) = compile_file_to_target(&source, &target) {
                println!("{}", e);
            }
        }
        else if let Some(package) = &args.package {
            if let Err(e) = compile_package_to_target(&package, &target) {
                println!("Failed to compile package {:?} ({})", package, e);
            }
        }
        else {
            match env::current_dir() {
                Ok(package) => {
                    if let Err(e) = compile_package_to_target(&package, &target) {
                        println!("{}", e);
                    }
                },
                Err(e) => {
                    println!("{}", e);
                },
            }
        }
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

fn run_command(args: &RunArgs) {
    match load_profile_from_directory(
        &args.profile,
        &args.profiles_directory,
    ) {
        Ok(profile) => {
            match choose_llm(&profile.llm) {
                Ok(llm) => {
                    match load_assembly_file(&args.assembly_file) {
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
                                },
                                Err(e) => {
                                    println!("Could not execute program ({})", e);
                                },
                            }
                        },
                        Err(e) => {
                            println!(
                                "Could not find assembly file {:?} ({})",
                                args.assembly_file,
                                e
                            );
                        }
                    }
                },
                Err(e) => {
                    println!("Specified large language model \"{}\" not found ({})", profile.llm, e);
                },
            }
        },
        Err(e) => println!("{}", e),
    }
}

fn new_command(args: &NewArgs) {
    match args.thing.as_str() {
        "profile" => {
            if let Err(e) = new_profile_command() {
                println!("{}", e);
            }
        },
        _ => {
            println!("{} is unknown for command new", args.thing);
        }
    }
}

fn new_profile_command() -> Result<(), String> {
    let name:            String = ask("Name your new profile")?;
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
}
