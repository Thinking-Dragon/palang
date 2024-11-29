use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum Instruction {
    Assign(String, String),
    Invoke(String, Vec<String>),
    Return(String),
}
