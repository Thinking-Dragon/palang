use palang_core::profile::Profile;
use reqwest::{header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE}, Client};
use serde_json::{json, Value};

use super::invokable_llm::InvokableLargeLanguageModel;

#[derive(Clone)]
pub struct GroqLargeLanguageModel {
    client: Client,
}

impl InvokableLargeLanguageModel for GroqLargeLanguageModel {
    async fn invoke(
        &self,
        system: &String,
        prompt: &String,
        profile: &Profile,
    ) -> Result<String, String> {
        if let Some(api_key) = &profile.api_key {
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
                "model": profile.model,
                "temperature": profile.temperature,
                "max_tokens": profile.max_tokens,
                "top_p": 1,
                "stream": false,
                "stop": null,
            });
    
            let mut headers = HeaderMap::new();
            headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
            headers.insert(AUTHORIZATION,
                HeaderValue::from_str(
                    &format!("Bearer {}", api_key).as_str()
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

            response
                .get("choices")
                .and_then(|choices| choices.get(0))
                .and_then(|choice| choice.get("message"))
                .and_then(|message| message.get("content"))
                .and_then(|content| content.as_str())
                .map(String::from)
                .ok_or_else(|| "Failed to extract content from response".to_string())
        }
        else {
            return Err("The chosen profile is missing an API key.".to_string())
        }

    }
}

impl GroqLargeLanguageModel {
    pub fn new() -> Self {
        GroqLargeLanguageModel {
            client: Client::new(),
        }
    }
}
