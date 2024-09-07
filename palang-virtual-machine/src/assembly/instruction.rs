#[derive(Debug, Clone)]
pub enum Instruction {
    Assign(String, String),
    Invoke(String, Vec<String>),
    Return(String),
}
