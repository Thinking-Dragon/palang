use super::parameter_info::ParameterInfo;

#[derive(Debug, Clone)]
pub struct FunctionInfo {
    pub parameters: Vec<ParameterInfo>,
    pub return_type: String,
}
