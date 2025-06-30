use pyo3::prelude::*;

use crate::program::{Program, RunnableProgram};

#[pyclass]
pub struct ProgramImporter {
    path: Vec<String>,
}

#[pymethods]
impl ProgramImporter {
    #[new]
    pub fn new() -> Self {
        ProgramImporter { path: Vec::new() }
    }

    fn __call__(&self, name: &str) -> Self {
        let path: Vec<String> = name.split("::").into_iter().map(|part| part.to_string()).collect();
        ProgramImporter { path }
    }

    fn target_profile(&self, profile_name: String) -> PyResult<RunnableProgram> {
        let task: String = self.path.join("::");
        let program: Program = Program::new(task);
        program.with_profile(profile_name)
    }
}
