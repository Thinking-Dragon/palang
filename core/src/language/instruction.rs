use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Instruction {
    Assign(String, String),
    Invoke(String, Vec<String>),
    Return(String),
}
