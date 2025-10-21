use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub mod context;
pub mod memory;

use context::AgentContext;
use memory::MemoryManager;

#[derive(Debug, Clone)]
pub struct SocioAgent {
    pub context: Arc<AgentContext>,
    pub memory: Arc<MemoryManager>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentRequest {
    pub message: String,
    pub session_id: String,
    pub platform: Option<String>,
    pub content_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentResponse {
    pub response: String,
    pub session_id: String,
    pub suggestions: Vec<String>,
    pub actions: Vec<AgentAction>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentAction {
    pub action_type: String,
    pub platform: Option<String>,
    pub content: Option<String>,
    pub metadata: serde_json::Value,
}

impl SocioAgent {
    pub fn new() -> Self {
        Self {
            context: Arc::new(AgentContext::new()),
            memory: Arc::new(MemoryManager::new()),
        }
    }

    pub async fn process_request(&self, request: &AgentRequest) -> Result<AgentResponse> {
        // Store the user message in memory
        self.memory.store_message(&request.session_id, &request.message, "user").await?;

        // Analyze the request and determine the appropriate response
        let analysis = self.analyze_request(request).await?;
        
        // Generate response based on analysis
        let response = self.generate_response(&analysis, request).await?;
        
        // Store the assistant response in memory
        self.memory.store_message(&request.session_id, &response, "assistant").await?;

        // Generate suggestions and actions
        let suggestions = self.generate_suggestions(&analysis).await?;
        let actions = self.generate_actions(&analysis, request).await?;

        Ok(AgentResponse {
            response,
            session_id: request.session_id.clone(),
            suggestions,
            actions,
        })
    }

    async fn analyze_request(&self, request: &AgentRequest) -> Result<RequestAnalysis> {
        let message = &request.message.to_lowercase();
        
        let analysis = RequestAnalysis {
            intent: self.detect_intent(message),
            platform: self.extract_platform(message, &request.platform),
            content_type: self.extract_content_type(message, &request.content_type),
            urgency: self.detect_urgency(message),
            sentiment: self.detect_sentiment(message),
        };

        Ok(analysis)
    }

    fn detect_intent(&self, message: &str) -> Intent {
        if message.contains("generate") || message.contains("create") || message.contains("write") {
            Intent::GenerateContent
        } else if message.contains("post") || message.contains("publish") || message.contains("share") {
            Intent::PostContent
        } else if message.contains("configure") || message.contains("setup") || message.contains("connect") {
            Intent::ConfigurePlatform
        } else if message.contains("help") || message.contains("how") || message.contains("what") {
            Intent::Help
        } else {
            Intent::GeneralChat
        }
    }

    fn extract_platform(&self, message: &str, explicit_platform: &Option<String>) -> Option<String> {
        if let Some(platform) = explicit_platform {
            return Some(platform.clone());
        }

        if message.contains("linkedin") {
            Some("linkedin".to_string())
        } else if message.contains("twitter") {
            Some("twitter".to_string())
        } else if message.contains("facebook") {
            Some("facebook".to_string())
        } else if message.contains("instagram") {
            Some("instagram".to_string())
        } else if message.contains("email") {
            Some("email".to_string())
        } else {
            None
        }
    }

    fn extract_content_type(&self, message: &str, explicit_type: &Option<String>) -> Option<String> {
        if let Some(content_type) = explicit_type {
            return Some(content_type.clone());
        }

        if message.contains("post") {
            Some("post".to_string())
        } else if message.contains("story") {
            Some("story".to_string())
        } else if message.contains("email") {
            Some("email".to_string())
        } else if message.contains("caption") {
            Some("caption".to_string())
        } else {
            Some("post".to_string())
        }
    }

    fn detect_urgency(&self, message: &str) -> Urgency {
        if message.contains("urgent") || message.contains("asap") || message.contains("immediately") {
            Urgency::High
        } else if message.contains("soon") || message.contains("quickly") {
            Urgency::Medium
        } else {
            Urgency::Low
        }
    }

    fn detect_sentiment(&self, message: &str) -> Sentiment {
        if message.contains("happy") || message.contains("excited") || message.contains("great") {
            Sentiment::Positive
        } else if message.contains("sad") || message.contains("disappointed") || message.contains("bad") {
            Sentiment::Negative
        } else {
            Sentiment::Neutral
        }
    }

    async fn generate_response(&self, analysis: &RequestAnalysis, request: &AgentRequest) -> Result<String> {
        match analysis.intent {
            Intent::GenerateContent => {
                self.generate_content_response(analysis, request).await
            }
            Intent::PostContent => {
                self.generate_post_response(analysis, request).await
            }
            Intent::ConfigurePlatform => {
                self.generate_config_response(analysis, request).await
            }
            Intent::Help => {
                self.generate_help_response().await
            }
            Intent::GeneralChat => {
                self.generate_chat_response(analysis, request).await
            }
        }
    }

    async fn generate_content_response(&self, analysis: &RequestAnalysis, _request: &AgentRequest) -> Result<String> {
        let platform = analysis.platform.as_deref().unwrap_or("general");
        let content_type = analysis.content_type.as_deref().unwrap_or("post");
        
        Ok(format!(
            "I'll help you generate {} content for {}. Let me create something engaging and platform-appropriate for you.",
            content_type, platform
        ))
    }

    async fn generate_post_response(&self, analysis: &RequestAnalysis, _request: &AgentRequest) -> Result<String> {
        let platform = analysis.platform.as_deref().unwrap_or("your chosen platform");
        
        Ok(format!(
            "I'll help you post content to {}. First, let me generate the content and then we can publish it.",
            platform
        ))
    }

    async fn generate_config_response(&self, analysis: &RequestAnalysis, _request: &AgentRequest) -> Result<String> {
        let platform = analysis.platform.as_deref().unwrap_or("social media platform");
        
        Ok(format!(
            "I'll help you configure {}. Please provide your API credentials and I'll set up the connection for you.",
            platform
        ))
    }

    async fn generate_help_response(&self) -> Result<String> {
        Ok("I'm your AI social media assistant! I can help you:\n\n• Generate content for LinkedIn, Twitter, Facebook, Instagram, and emails\n• Post content to your social media platforms\n• Configure API connections\n• Provide content suggestions\n\nWhat would you like me to help you with today?".to_string())
    }

    async fn generate_chat_response(&self, _analysis: &RequestAnalysis, _request: &AgentRequest) -> Result<String> {
        Ok("I understand you'd like to chat! I'm here to help with your social media content needs. How can I assist you today?".to_string())
    }

    async fn generate_suggestions(&self, analysis: &RequestAnalysis) -> Result<Vec<String>> {
        let mut suggestions = Vec::new();
        
        match analysis.intent {
            Intent::GenerateContent => {
                suggestions.push("Generate LinkedIn post".to_string());
                suggestions.push("Create Twitter thread".to_string());
                suggestions.push("Write Instagram caption".to_string());
                suggestions.push("Compose email".to_string());
            }
            Intent::PostContent => {
                suggestions.push("Post to LinkedIn".to_string());
                suggestions.push("Share on Twitter".to_string());
                suggestions.push("Publish on Facebook".to_string());
                suggestions.push("Upload to Instagram".to_string());
            }
            Intent::ConfigurePlatform => {
                suggestions.push("Configure LinkedIn API".to_string());
                suggestions.push("Setup Twitter API".to_string());
                suggestions.push("Connect Facebook API".to_string());
                suggestions.push("Link Instagram API".to_string());
            }
            _ => {
                suggestions.push("Generate content".to_string());
                suggestions.push("Post to social media".to_string());
                suggestions.push("Configure platforms".to_string());
                suggestions.push("Get help".to_string());
            }
        }
        
        Ok(suggestions)
    }

    async fn generate_actions(&self, analysis: &RequestAnalysis, request: &AgentRequest) -> Result<Vec<AgentAction>> {
        let mut actions = Vec::new();
        
        if let Some(platform) = &analysis.platform {
            actions.push(AgentAction {
                action_type: "generate_content".to_string(),
                platform: Some(platform.clone()),
                content: Some(request.message.clone()),
                metadata: serde_json::json!({
                    "content_type": analysis.content_type,
                    "urgency": analysis.urgency,
                    "sentiment": analysis.sentiment
                }),
            });
        }
        
        Ok(actions)
    }
}

#[derive(Debug)]
struct RequestAnalysis {
    intent: Intent,
    platform: Option<String>,
    content_type: Option<String>,
    urgency: Urgency,
    sentiment: Sentiment,
}

#[derive(Debug, Serialize, Deserialize)]
enum Intent {
    GenerateContent,
    PostContent,
    ConfigurePlatform,
    Help,
    GeneralChat,
}

#[derive(Debug, Serialize, Deserialize)]
enum Urgency {
    High,
    Medium,
    Low,
}

#[derive(Debug, Serialize, Deserialize)]
enum Sentiment {
    Positive,
    Neutral,
    Negative,
}
