use palang_core::{language::assembly_source::{AssemblySource, WrappedAssembly}, project::WrappedProject, storage::NamedData};
use palang_virtual_machine::assembly::{
    assembly::Assembly,
    dependency_tree::AssemblyDependencyNode,
    loader::load_assembly
};
use tabled::{
    settings::{
        object::Rows,
        themes::Colorization,
        Color,
        Style
    },
    Table,
    Tabled
};

pub fn pretty_print_projects(
    projects: &Vec<NamedData<WrappedProject>>,
) -> String {
    if projects.is_empty() {
        "No project was found".to_string()
    }
    else {
        Table::new(
            projects.into_iter()
                .map(
                    |named_project|
                    PrintableProject::from_named(named_project)
                )
                .collect::<Vec<PrintableProject>>()
        ).with(Style::modern_rounded())
         .with(Colorization::exact([Color::BOLD], Rows::first()))
         .to_string()
    }
}

pub fn pretty_print_project(project: &NamedData<WrappedProject>) -> Result<String, String> {
    let mut print: String = String::new();

    print += format!("Name: {}\n", project.name).as_str();
    print += pretty_print_assemblies(&project.data.assemblies)?.as_str();

    Ok(print)
}

pub fn pretty_print_assembly_sources(assemblies: &Vec<WrappedAssembly>) -> Result<String, String> {
    if assemblies.is_empty() {
        Err("No assembly was found".to_string())
    }
    else {
        let assembly_sources: Vec<AssemblySource> = assemblies.iter()
        .map(|assembly| assembly.source.clone())
        .collect();

        let mut text: String = String::new();

        text += "Sources:\n";
        let printable_assemblies: Vec<PrintableAssemblySource> = assembly_sources.iter()
            .filter_map(
                |assembly|
                match PrintableAssemblySource::from_assembly(assembly) {
                    Ok(printable_assembly) => Some(printable_assembly),
                    Err(_) => None,
                }
            )
            .collect();

        text += Table::new(printable_assemblies)
            .with(Style::modern_rounded())
            .with(Colorization::exact([Color::BOLD], Rows::first()))
            .to_string().as_str();

        Ok(text)
    }
}

pub fn pretty_print_assemblies(assemblies: &Vec<WrappedAssembly>) -> Result<String, String> {
    if assemblies.is_empty() {
        Err("No assembly was found".to_string())
    }
    else {
        let assemblies: Vec<Assembly> = assemblies.iter()
            .filter_map(
                |assembly|
                match load_assembly(&assembly.instructions) {
                    Ok(assembly) => Some(assembly),
                    Err(_) => None,
                }
            )
            .collect();

        let dependency_tree: AssemblyDependencyNode = AssemblyDependencyNode::from_assemblies(&assemblies);

        dependency_tree.to_string()
    }
}

#[derive(Debug, Tabled)]
struct PrintableProject {
    pub name: String,
    pub nb_assemblies: usize,
}

impl PrintableProject {
    pub fn from_named(named: &NamedData<WrappedProject>) -> Self {
        PrintableProject {
            name: named.name.clone(),
            nb_assemblies: named.data.assemblies.len(),
        }
    }
}

#[derive(Debug, Tabled)]
struct PrintableAssemblySource {
    pub source: String,
    pub details: String,
}

impl PrintableAssemblySource {
    pub fn from_assembly(assembly_source: &AssemblySource) -> Result<Self, String> {      
        match &assembly_source {
            AssemblySource::Path(path) => {
                let details: String = match assembly_source.resolve_assembly() {
                    Ok(assembly) => {
                        let dependency_tree: AssemblyDependencyNode = AssemblyDependencyNode::from_assembly(
                            &load_assembly(&assembly.instructions)?
                        );
                        format!(
                            "Path: {}\n{}",
                            path.clone(),
                            dependency_tree.to_string()?,
                        )
                    },
                    Err(e) => {
                        format!("Path: {}\nCould not resolve assembly: {}", path.clone(), e)
                    },
                };

                Ok(PrintableAssemblySource { source: "Remote".to_string(), details })
            },
            AssemblySource::Code(_) => {
                let details: String = match assembly_source.resolve_assembly() {
                    Ok(assembly) => {
                        AssemblyDependencyNode::from_assembly(
                            &load_assembly(&assembly.instructions)?
                        ).to_string()?
                    },
                    Err(e) => {
                        format!("Could not resolve assembly: {}", e)
                    },
                };

                Ok(PrintableAssemblySource { source: "Local".to_string(), details })
            },
        }
    }
}
