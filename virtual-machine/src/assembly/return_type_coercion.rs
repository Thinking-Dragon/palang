use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum ReturnTypeCoercion {
    Embbedded(String),
    FromCaller,
    NotCoerced,
}
