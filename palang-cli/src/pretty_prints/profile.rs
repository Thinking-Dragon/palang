use palang_server::api::v1::{models::profile::Profile, services::storage::NamedData};
use tabled::{settings::{object::Rows, themes::Colorization, Color, Style}, Table, Tabled};

pub fn pretty_print_profiles(
    profiles: &Vec<NamedData<Profile>>,
) -> String {
    if profiles.is_empty() {
        "No profile was found".to_string()
    }
    else {
        Table::new(
            profiles.into_iter()
                .map(
                    |named_profile|
                    PrintableProfile::from_named(named_profile)
                )
                .collect::<Vec<PrintableProfile>>()
        ).with(Style::modern_rounded())
         .with(Colorization::exact([Color::BOLD], Rows::first()))
         .to_string()
    }
}

#[derive(Debug, Tabled)]
struct PrintableProfile {
    pub name: String,
    pub llm: String,
    pub model: String,
    pub temperature: f32,
    pub max_tokens: u32,
}

impl PrintableProfile {
    pub fn from_named(named: &NamedData<Profile>) -> Self {
        PrintableProfile {
            name: named.name.clone(),
            llm: named.data.llm.clone(),
            model: named.data.model.clone(),
            temperature: named.data.temperature,
            max_tokens: named.data.max_tokens,
        }
    }
}
