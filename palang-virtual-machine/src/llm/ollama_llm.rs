use reqwest::{Client, header::{HeaderMap, HeaderValue, CONTENT_TYPE}};
use serde_json::{json, Value};

use super::{invokable_llm::InvokableLargeLanguageModel, model_settings::ModelSettings};

#[derive(Clone)]
pub struct OllamaLargeLanguageModel {
    client: Client,
    base_url: String,
}

impl InvokableLargeLanguageModel for OllamaLargeLanguageModel {
    async fn invoke(
        &self,
        system: &String,
        prompt: &String,
        settings: &ModelSettings,
    ) -> Result<String, String> {
        let body = json!({
            "messages": [
                {
                    "role": "system",
                    "content": system,
                },
                {
                    "role": "user",
                    "content": prompt,
                }
            ],
            "model": settings.model,
            "temperature": settings.temperature,
            "max_tokens": settings.max_tokens,
            "stream": false,
        });

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let response: Value = self.client
            .post(format!("{}/v1/chat/completions", self.base_url))
            .headers(headers)
            .json(&body)
            .send()
            .await
            .map_err(|e| e.to_string())?
            .json()
            .await
            .map_err(|e| e.to_string())?;

        // Extract the content from the message
        response
            .get("message")
            .and_then(|message| message.get("content"))
            .and_then(|content| content.as_str())
            .map(String::from)
            .ok_or_else(|| "Failed to extract content from response".to_string())
    }
}

impl OllamaLargeLanguageModel {
    pub fn new(base_url: &String) -> Self {
        OllamaLargeLanguageModel {
            client: Client::new(),
            base_url: base_url.clone(),
        }
    }
}
