use palang_core::{
    project::WrappedProject,
    storage::NamedData
};

use crate::{
    pretty_prints::project::pretty_print_projects,
    server_proxy::ServerProxy
};

pub fn projects_command() -> Result<(), String> {
    let projects: Vec<NamedData<WrappedProject>> = ServerProxy::find_server()?.get_projects()?;
    println!("{}", pretty_print_projects(&projects));
    Ok(())
}
