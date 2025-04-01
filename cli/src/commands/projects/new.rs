use clap::Parser;

use palang_core::{
    project::Project,
    storage::name_data
};

use crate::server_proxy::ServerProxy;

#[derive(Debug, Parser)]
pub struct NewProjectArgs {
    name: String,
}

pub fn new_project_command(args: &NewProjectArgs) -> Result<(), String> {
    ServerProxy::find_server()?
        .add_project(
            &name_data(
                args.name.clone(),
                Project::new()
            )
        )?;

    println!("Project added successfully");
    Ok(())
}
