use pyo3::prelude::*;

use crate::model::Model;

#[pyclass]
pub struct ModelImporter;

#[pymethods]
impl ModelImporter {
    #[new]
    pub fn new() -> Self {
        ModelImporter
    }

    fn __call__(&self, path: &str) -> Model {
        Model::new(path.replace("::", "/"))
    }
}
