use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone)]
pub struct Model {
    pub path: String,
}

#[pymethods]
impl Model {
    #[new]
    pub fn new(path: String) -> Self {
        Model { path }
    }
}
