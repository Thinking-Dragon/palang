use palang_core::profile::Profile;

use super::{groq_llm::GroqLargeLanguageModel, invokable_llm::InvokableLargeLanguageModel, ollama_llm::OllamaLargeLanguageModel};

#[derive(Clone)]
pub enum LargeLanguageModel {
    Groq(GroqLargeLanguageModel),
    Ollama(OllamaLargeLanguageModel),
}

impl LargeLanguageModel {
    pub fn new_groq() -> Self {
        LargeLanguageModel::Groq(GroqLargeLanguageModel::new())
    }

    pub fn new_ollama(base_url: &String) -> Self {
        LargeLanguageModel::Ollama(OllamaLargeLanguageModel::new(base_url))
    }

    pub async fn invoke(
        &self,
        system: &String,
        prompt: &String,
        profile: &Profile,
    ) -> Result<String, String> {
        match self {
            LargeLanguageModel::Groq(llm) => llm.invoke(system, prompt, profile).await,
            LargeLanguageModel::Ollama(llm) => llm.invoke(system, prompt, profile).await,
        }
    }
}
