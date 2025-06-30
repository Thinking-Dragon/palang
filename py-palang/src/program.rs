use std::{env, fs, path::PathBuf, process::exit};

use palang_compiler::compile_file;
use palang_core::profile::{load_profile, Profile};
use palang_virtual_machine::{assembly::{assembly::Assembly, loader::load_assembly, return_type_coercion::ReturnTypeCoercion}, boot_machine, choose_llm, llm::llm::LargeLanguageModel, virtualization::virtual_machine::VirtualMachine};
use pyo3::prelude::*;
use tokio::runtime::Runtime;

use crate::model::Model;

#[pyclass]
pub struct Program {
    task: String,
}

#[pymethods]
impl Program {
    #[new]
    pub fn new(task: String) -> Self {
        Program { task }
    }

    pub fn with_profile(&self, profile_name: String) -> PyResult<RunnableProgram> {
        let profile_path: PathBuf = env::current_dir()?
            .join("palang")
            .join("profiles")
            .join(format!("{}.yaml", profile_name));

        let profile: Result<Profile, String> = load_profile(&profile_path);
        if let Err(_) = profile {
            println!("Could not load profile, make sure the profile name matches your profile's file name");
            exit(1);
        }
        let profile: Profile = profile.unwrap();

        let llm: Result<LargeLanguageModel, String> = choose_llm(&profile.llm);
        if let Err(e) = llm {
            println!("{}", e);
            exit(1);
        }
        let llm: LargeLanguageModel = llm.unwrap();

        let program_path: PathBuf = env::current_dir()?
            .join("hello.palang");

        let source_code: String = fs::read_to_string(program_path)?;
        let assembly_code: String = compile_file(&source_code).unwrap();
        let assembly: Assembly = load_assembly(&assembly_code).unwrap();

        let mut vm: VirtualMachine = boot_machine(&llm);
        vm.load_assembly(&assembly);

        Ok(RunnableProgram::new(profile, vm, self.task.clone(), None))
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct RunnableProgram {
    profile: Profile,
    vm: VirtualMachine,
    task: String,
    coerced_type: Option<String>,
}

impl RunnableProgram {
    pub fn new(profile: Profile, vm: VirtualMachine, task: String, coerced_type: Option<String>) -> Self {
        RunnableProgram { profile, vm, task, coerced_type }
    }
}

#[pymethods]
impl RunnableProgram {
    pub fn __call__(&mut self) -> PyResult<String> {
        let runtime: Runtime = tokio::runtime::Runtime::new().unwrap();

        let coerced_type: ReturnTypeCoercion = match &self.coerced_type {
            Some(model_path) => ReturnTypeCoercion::Embbedded(model_path.clone()),
            None => ReturnTypeCoercion::NotCoerced,
        };

        let result: Result<String, String> = runtime.block_on(async {
            self.vm.execute(
                &self.task.replace("::", "/"),
                &Vec::new(),
                &coerced_type,
                &self.profile,
            ).await.await
        });

        Ok(result.unwrap())
    }

    pub fn cast_as(&mut self, model: Model) -> PyResult<Self> {
        self.coerced_type = Some(model.path);
        Ok(self.clone())
    }
}
