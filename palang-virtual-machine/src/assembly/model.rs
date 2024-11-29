use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Model {
    pub name: String,
    pub text: String,
}
