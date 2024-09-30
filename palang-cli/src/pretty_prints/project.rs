use palang_server::api::v1::{models::{assembly::AssemblySource, project::Project}, services::storage::NamedData};
use tabled::{settings::{object::Rows, themes::Colorization, Color, Style}, Table, Tabled};

pub fn pretty_print_projects(
    projects: &Vec<NamedData<Project>>,
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

pub fn pretty_print_project(project: &NamedData<Project>) -> String {
    let mut print: String = String::new();

    print += format!("Name: {}\n", project.name).as_str();
    print += format!("Assemblies:\n").as_str();
    print += pretty_print_assemblies(&project.data.assemblies).as_str();

    print
}

pub fn pretty_print_assemblies(assemblies: &Vec<AssemblySource>) -> String {
    if assemblies.is_empty() {
        "No assembly was found".to_string()
    }
    else {
        Table::new(
            assemblies.into_iter()
                .map(
                    |assembly_source|
                    PrintableAssembly::from_assembly(assembly_source)
                )
                .collect::<Vec<PrintableAssembly>>()
        ).with(Style::modern_rounded())
         .with(Colorization::exact([Color::BOLD], Rows::first()))
         .to_string()
    }
}

#[derive(Debug, Tabled)]
struct PrintableProject {
    pub name: String,
    pub nb_assemblies: usize,
}

impl PrintableProject {
    pub fn from_named(named: &NamedData<Project>) -> Self {
        PrintableProject {
            name: named.name.clone(),
            nb_assemblies: named.data.assemblies.len(),
        }
    }
}

#[derive(Debug, Tabled)]
struct PrintableAssembly {
    pub source: String,
    pub details: String,
}

impl PrintableAssembly {
    pub fn from_assembly(assembly: &AssemblySource) -> Self {
        match &assembly {
            AssemblySource::Path(path) => {
                PrintableAssembly {
                    source: "Remote".to_string(),
                    details: path.clone(),
                }
            },
            AssemblySource::Code(_) => {
                PrintableAssembly {
                    source: "Local".to_string(),
                    details: "".to_string(),
                }
            },
        }
    }
}
