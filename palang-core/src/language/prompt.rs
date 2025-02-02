use serde::{Deserialize, Serialize};

use super::parameter::Parameter;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Prompt {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: String,
    pub text: String,
}
