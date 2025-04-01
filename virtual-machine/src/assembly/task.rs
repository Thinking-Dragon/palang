use super::{function::Function, prompt::Prompt};

pub enum Task {
    Prompt(Prompt),
    Function(Function),
}
