use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct GeminiRequest {
    pub contents: Vec<Content>,
    pub generation_config: Option<GenerationConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Content {
    pub parts: Vec<Part>,
    pub role: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Part {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerationConfig {
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub top_k: Option<i32>,
    pub max_output_tokens: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeminiResponse {
    pub candidates: Vec<Candidate>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Candidate {
    pub content: Content,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Clone)]
pub struct GeminiClient {
    client: Client,
    api_key: String,
    model: String,
    base_url: String,
}

impl GeminiClient {
    pub fn new() -> Result<Self> {
        let api_key = env::var("GEMINI_API_KEY")
            .map_err(|_| anyhow!("GEMINI_API_KEY environment variable not set"))?;
        
        let model = env::var("GEMINI_MODEL").unwrap_or_else(|_| "gemini-1.5-flash".to_string());
        let base_url = "https://generativelanguage.googleapis.com/v1beta".to_string();
        
        Ok(Self {
            client: Client::new(),
            api_key,
            model,
            base_url,
        })
    }

    pub async fn generate_content(
        &self,
        prompt: &str,
        temperature: Option<f32>,
    ) -> Result<String> {
        let request = GeminiRequest {
            contents: vec![Content {
                parts: vec![Part {
                    text: prompt.to_string(),
                }],
                role: Some("user".to_string()),
            }],
            generation_config: Some(GenerationConfig {
                temperature,
                top_p: Some(0.95),
                top_k: Some(40),
                max_output_tokens: Some(8192),
            }),
        };

        let url = format!(
            "{}/models/{}:generateContent?key={}",
            self.base_url, self.model, self.api_key
        );

        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow!("Gemini API error: {}", error_text));
        }

        let gemini_response: GeminiResponse = response.json().await?;
        
        if let Some(candidate) = gemini_response.candidates.first() {
            if let Some(part) = candidate.content.parts.first() {
                Ok(part.text.clone())
            } else {
                Err(anyhow!("No content generated"))
            }
        } else {
            Err(anyhow!("No candidates returned"))
        }
    }

    pub async fn generate_social_media_content(
        &self,
        platform: &str,
        topic: &str,
        tone: &str,
        length: &str,
    ) -> Result<String> {
        let prompt = format!(
            "Generate a {} post for {} about '{}' with a {} tone. Keep it {} characters or less. Make it engaging and platform-appropriate.",
            platform, platform, topic, tone, length
        );

        self.generate_content(&prompt, Some(0.7)).await
    }

    pub async fn generate_email_content(
        &self,
        subject: &str,
        recipient: &str,
        purpose: &str,
        tone: &str,
    ) -> Result<String> {
        let prompt = format!(
            "Generate a professional email with subject '{}' for {} about {}. Use a {} tone. Include proper greeting and closing.",
            subject, recipient, purpose, tone
        );

        self.generate_content(&prompt, Some(0.6)).await
    }
}
