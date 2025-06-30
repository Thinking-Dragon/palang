use pyo3::prelude::*;

use crate::{model::Model, model_importer::ModelImporter, program::{Program, RunnableProgram}, program_importer::ProgramImporter};

mod model_importer;
mod model;

mod program_importer;
mod program;

#[pymodule]
fn palang(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ModelImporter>()?;
    m.add_class::<Model>()?;
    m.add_class::<ProgramImporter>()?;
    m.add_class::<Program>()?;
    m.add_class::<RunnableProgram>()?;

    let model_importer: ModelImporter = ModelImporter::new();
    m.add("import_model", model_importer)?;

    let program_importer: ProgramImporter = ProgramImporter::new();
    m.add("import_program", program_importer)?;

    Ok(())
}
