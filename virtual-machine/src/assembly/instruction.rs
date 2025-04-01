use serde::Serialize;

use super::return_type_coercion::ReturnTypeCoercion;

#[derive(Debug, Clone, Serialize)]
pub enum Instruction {
    Assign(String, String),
    Invoke(String, Vec<String>, ReturnTypeCoercion),
    Return(String),
}
