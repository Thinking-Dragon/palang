use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Parameter {
    pub name: String,
}
