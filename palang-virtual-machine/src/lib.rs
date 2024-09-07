use std::{fs, path::PathBuf};

use assembly::{assembly::Assembly, loader::load_assembly};
use llm::llm::LargeLanguageModel;
use virtualization::virtual_machine::VirtualMachine;

pub mod assembly;
pub mod virtualization;
pub mod llm;

pub fn load_assembly_file(file: &PathBuf) -> Result<Assembly, String> {
    let assembly_code: String = fs::read_to_string(file)
                                   .map_err(|e| e.to_string())?;

    load_assembly(&assembly_code)
}

pub fn choose_llm(llm: &String) -> Result<LargeLanguageModel, String> {
    match llm.to_lowercase().as_str() {
        "groq" => {
            let authorization_token: String = std::env::var("GROQ_AUTHORIZATION_TOKEN")
                                                       .map_err(|e| e.to_string())?;
            Ok(LargeLanguageModel::new_groq(&authorization_token))
        },
        "ollama" => {
            let base_url: String = std::env::var("OLLAMA_BASE_URL")
                                            .map_err(|e| e.to_string())?;
            Ok(LargeLanguageModel::new_ollama(&base_url))
        },
        _ => Err(format!("Large Language Model \"{}\" not found", llm.to_lowercase())),
    }
}

pub fn boot_machine(llm: &LargeLanguageModel) -> VirtualMachine {
    let vm: VirtualMachine = VirtualMachine::new(llm);

    // Todo: load std assemblies into vm.

    vm
}
