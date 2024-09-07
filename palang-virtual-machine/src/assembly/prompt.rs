use super::parameter::Parameter;

#[derive(Debug, Clone)]
pub struct Prompt {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: String,
    pub text: String,
}
