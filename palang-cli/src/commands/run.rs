use std::{fs, path::PathBuf};

use clap::Parser;
use palang_compiler::compile_file;
use palang_virtual_machine::{assembly::{assembly::Assembly, loader::load_assembly}, boot_machine, choose_llm, load_assembly_file, virtualization::virtual_machine::VirtualMachine};
use tokio::runtime::Runtime;

use crate::server_proxy::models::profile::load_profile_from_directory;

#[derive(Debug, Parser)]
pub struct RunArgs {
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

pub fn run_command(args: &RunArgs) -> Result<(), String> {
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
