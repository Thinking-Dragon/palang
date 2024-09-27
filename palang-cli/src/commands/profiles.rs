use std::path::PathBuf;

use clap::{Parser, Subcommand};
use palang_server::api::v1::{
    models::profile::{
        import_profile,
        load_profile,
        Profile,
        ProfileAlias
    },
    services::storage::{name_data, NamedData}
};

use crate::{
    dialog_utils::ask, pretty_prints::profile::pretty_print_profiles, server_proxy::ServerProxy
};

#[derive(Debug, Parser)]
pub struct ProfilesArgs {
    #[command(subcommand)]
    command: Option<ProfilesCommand>,
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
        Some(command) => {
            match command {
                ProfilesCommand::New(new_args) => {
                    new_profile_command(&new_args.name, &new_args.from)
                },
                ProfilesCommand::NewAlias(alias_args) => {
                    new_profile_alias_command(&alias_args.name, &alias_args.r#for)
                },
            }
        },
        None => {
            let profiles: Vec<NamedData<Profile>> = ServerProxy::find_server()?.get_profiles()?;
            println!("{}", pretty_print_profiles(&profiles));
            Ok(())
        },
    }
}

fn new_profile_command(name: &String, from: &Option<PathBuf>) -> Result<(), String> {
    if let Some(from) = from {
        let profile: Profile = load_profile(from)?;

        if ServerProxy::is_connected() {
            ServerProxy::find_server()?.add_profile(
                &name_data(
                    name.clone(),
                    profile,
                )
            )
        }
        else {
            import_profile(name, &profile)
        }
    }
    else {
        new_profile_dialog(name)
    }
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

    if ServerProxy::is_connected() {
        ServerProxy::find_server()?.add_profile(
            &name_data(
                name.clone(),
                profile,
            )
        )
    }
    else {
        import_profile(name, &profile)
    }
}

fn new_profile_alias_command(name: &String, r#for: &String) -> Result<(), String> {
    ServerProxy::find_server()?
        .add_profile_alias(&ProfileAlias::new(name.clone(), r#for.clone()))
}
