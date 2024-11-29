use std::path::PathBuf;

use clap::Parser;
use palang_core::profile::load_profile_from_directory;
use palang_virtual_machine::{
    boot_machine,
    choose_llm,
    virtualization::virtual_machine::VirtualMachine
};
use tokio::runtime::Runtime;

use crate::assembly_utils::get_assembly_from_file;

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
                    match get_assembly_from_file(&args.assembly_file) {
                        Ok(asm) => {
                            let mut vm: VirtualMachine = boot_machine(&llm);
                            vm.load_assembly(&asm);

                            let runtime: Runtime = tokio::runtime::Runtime::new().unwrap();
                            let result: Result<String, String> = runtime.block_on(async {
                                vm.execute(
                                    &args.task,
                                    &args.args,
                                    &profile
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
