use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::{dialog_utils::ask, profile::{import_profile, load_profile, Profile}, server_proxy::ServerProxy};

#[derive(Debug, Parser)]
pub struct ProfilesArgs {
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

pub fn profiles_command(args: &ProfilesArgs) -> Result<(), String> {
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
    let mut proxy: ServerProxy = ServerProxy::find_server()?;
    
    proxy.add_profile_alias(name, r#for);
    
    Ok(())
}
