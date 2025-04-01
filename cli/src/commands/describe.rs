use std::path::PathBuf;

use clap::Parser;

use crate::assembly_utils::get_assembly_from_file;

#[derive(Debug, Parser)]
pub struct DescribeArgs {
    #[arg(value_name = "Path to the file which's content you want to describe")]
    source_file: PathBuf,

    #[clap(long, action)]
    json: bool,
}

pub fn describe_command(args: &DescribeArgs) -> Result<(), String> {
    let assembly = get_assembly_from_file(&args.source_file)?;

    let description: String = match args.json {
        true => {
            serde_json::to_string_pretty(&assembly).map_err(|e| e.to_string())?
        },
        false => {
            assembly.get_dependency_tree().to_string()?
        },
    };

    println!("{}", description);
    Ok(())
}
