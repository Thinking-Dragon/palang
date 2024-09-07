use reqwest::{header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE}, Client};
use serde_json::{json, Value};

use super::{invokable_llm::InvokableLargeLanguageModel, model_settings::ModelSettings};

#[derive(Clone)]
pub struct GroqLargeLanguageModel {
    client: Client,
    authorization_token: String,
}

impl InvokableLargeLanguageModel for GroqLargeLanguageModel {
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
            "top_p": 1,
            "stream": false,
            "stop": null,
        });

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(AUTHORIZATION,
            HeaderValue::from_str(
                &format!("Bearer {}", self.authorization_token).as_str()
            ).map_err(|e| e.to_string())?
        );

        let response: Value = self.client
            .post("https://api.groq.com/openai/v1/chat/completions")
            .headers(headers)
            .json(&body)
            .send()
            .await
            .map_err(|e| e.to_string())?
            .json()
            .await
            .map_err(|e| e.to_string())?;

        // Extract the content from the first choice's message
        response
            .get("choices")
            .and_then(|choices| choices.get(0))
            .and_then(|choice| choice.get("message"))
            .and_then(|message| message.get("content"))
            .and_then(|content| content.as_str())
            .map(String::from)
            .ok_or_else(|| "Failed to extract content from response".to_string())
    }
}

impl GroqLargeLanguageModel {
    pub fn new(authorization_token: &String) -> Self {
        GroqLargeLanguageModel {
            client: Client::new(),
            authorization_token: authorization_token.clone(),
        }
    }
}
