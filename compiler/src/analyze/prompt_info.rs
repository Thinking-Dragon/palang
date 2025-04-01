use super::parameter_info::ParameterInfo;

#[derive(Debug, Clone)]
pub struct PromptInfo {
    pub parameters: Vec<ParameterInfo>,
    pub return_type: String,
}
