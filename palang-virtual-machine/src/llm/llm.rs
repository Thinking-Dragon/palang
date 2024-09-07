use super::{groq_llm::GroqLargeLanguageModel, invokable_llm::InvokableLargeLanguageModel, model_settings::ModelSettings, ollama_llm::OllamaLargeLanguageModel};

#[derive(Clone)]
pub enum LargeLanguageModel {
    Groq(GroqLargeLanguageModel),
    Ollama(OllamaLargeLanguageModel),
}

impl LargeLanguageModel {
    pub fn new_groq(authorization_token: &String) -> Self {
        LargeLanguageModel::Groq(GroqLargeLanguageModel::new(authorization_token))
    }

    pub fn new_ollama(base_url: &String) -> Self {
        LargeLanguageModel::Ollama(OllamaLargeLanguageModel::new(base_url))
    }

    pub async fn invoke(
        &self,
        system: &String,
        prompt: &String,
        settings: &ModelSettings,
    ) -> Result<String, String> {
        match self {
            LargeLanguageModel::Groq(llm) => llm.invoke(system, prompt, settings).await,
            LargeLanguageModel::Ollama(llm) => llm.invoke(system, prompt, settings).await,
        }
    }
}
