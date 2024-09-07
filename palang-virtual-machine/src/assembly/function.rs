use super::{instruction::Instruction, parameter::Parameter};

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: String,
    pub instructions: Vec<Instruction>,
}
