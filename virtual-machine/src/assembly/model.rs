use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum Model {
    Litteral(LitteralModel),
    Enumerable(EnumerableModel),
}

#[derive(Debug, Clone, Serialize)]
pub struct LitteralModel {
    pub name: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct EnumerableModel {
    pub name: String,
    pub possible_models: Vec<String>,
}

impl From<LitteralModel> for Model {
    fn from(model: LitteralModel) -> Self {
        Model::Litteral(model)
    }
}

impl From<EnumerableModel> for Model {
    fn from(model: EnumerableModel) -> Self {
        Model::Enumerable(model)
    }
}
