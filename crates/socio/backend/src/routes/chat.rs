//! Chat API handler for the Socio backend.
//!
//! This handler processes chat requests and generates responses using
//! the LLM service. It follows the project's coding standards with
//! proper error handling and response formatting.

use actix_web::{web, HttpResponse, Result as ActixResult};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::types::app_state::AppState;

#[derive(Debug, Deserialize, ToSchema)]
pub struct ChatRequest {
    pub message: String,
    pub session_id: Option<String>,
    pub platform: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ChatResponse {
    pub response: String,
    pub session_id: String,
    pub suggestions: Vec<String>,
}

/// Handles chat requests and generates AI responses.
///
/// # Arguments
///
/// * `req` - The chat request data
/// * `app_state` - The application state containing LLM client
///
/// # Returns
///
/// An HTTP response containing the chat response.
#[utoipa::path(
    post,
    path = "/api/chat",
    request_body = ChatRequest,
    responses(
        (status = 200, description = "Chat response generated successfully", body = ChatResponse),
        (status = 400, description = "Bad request", body = String),
        (status = 500, description = "Internal server error", body = String)
    ),
    tag = "chat"
)]
pub async fn handle_chat(
    req: web::Json<ChatRequest>,
    app_state: web::Data<AppState>,
) -> ActixResult<HttpResponse> {
    let session_id = req.session_id.clone().unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
    
    // Generate response using LLM
    let response = match generate_chat_response(&req.message).await {
        Ok(response) => response,
        Err(e) => {
            tracing::error!("Failed to generate chat response: {}", e);
            "I apologize, but I'm having trouble generating a response right now. Please try again.".to_string()
        }
    };
    
    // Generate suggestions based on the request
    let suggestions = generate_suggestions(&req.message, &req.platform);
    
    let chat_response = ChatResponse {
        response,
        session_id: session_id.clone(),
        suggestions,
    };
    
    Ok(HttpResponse::Ok().json(chat_response))
}

/// Generates a chat response using the LLM client.
///
/// # Arguments
///
/// * `message` - The user's message
///
/// # Returns
///
/// A `Result` containing the generated response.
async fn generate_chat_response(
    message: &str,
) -> anyhow::Result<String> {
    // TODO: Use proper LLM client method when available
    // For now, return a mock response
    Ok(format!("I understand you want help with: '{}'. I can help you create content for LinkedIn, Twitter, Facebook, Instagram, and other platforms. What specific type of content would you like to create?", message))
}

/// Generates suggestions based on the user's message and platform.
///
/// # Arguments
///
/// * `message` - The user's message
/// * `platform` - Optional platform preference
///
/// # Returns
///
/// A vector of suggestion strings.
fn generate_suggestions(message: &str, platform: &Option<String>) -> Vec<String> {
    let mut suggestions = Vec::new();
    
    let message_lower = message.to_lowercase();
    
    if message_lower.contains("generate") || message_lower.contains("create") {
        suggestions.push("Generate LinkedIn post".to_string());
        suggestions.push("Create Twitter thread".to_string());
        suggestions.push("Write Instagram caption".to_string());
    }
    
    if message_lower.contains("post") || message_lower.contains("publish") {
        suggestions.push("Post to LinkedIn".to_string());
        suggestions.push("Share on Twitter".to_string());
        suggestions.push("Publish on Facebook".to_string());
    }
    
    if message_lower.contains("configure") || message_lower.contains("setup") {
        suggestions.push("Configure LinkedIn API".to_string());
        suggestions.push("Setup Twitter API".to_string());
        suggestions.push("Connect Facebook API".to_string());
    }
    
    // Add platform-specific suggestions
    if let Some(platform) = platform {
        match platform.as_str() {
            "linkedin" => {
                suggestions.push("Generate professional LinkedIn post".to_string());
                suggestions.push("Create LinkedIn article".to_string());
            }
            "twitter" => {
                suggestions.push("Create Twitter thread".to_string());
                suggestions.push("Generate tweet".to_string());
            }
            "facebook" => {
                suggestions.push("Create Facebook post".to_string());
                suggestions.push("Generate Facebook story".to_string());
            }
            "instagram" => {
                suggestions.push("Create Instagram caption".to_string());
                suggestions.push("Generate Instagram story".to_string());
            }
            _ => {}
        }
    }
    
    // Default suggestions if none were added
    if suggestions.is_empty() {
        suggestions.push("Generate content".to_string());
        suggestions.push("Post to social media".to_string());
        suggestions.push("Configure platforms".to_string());
        suggestions.push("Get help".to_string());
    }
    
    suggestions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_suggestions_generate_content() {
        let suggestions = generate_suggestions("I want to generate content", &None);
        assert!(suggestions.contains(&"Generate LinkedIn post".to_string()));
        assert!(suggestions.contains(&"Create Twitter thread".to_string()));
    }

    #[test]
    fn test_generate_suggestions_post_content() {
        let suggestions = generate_suggestions("I want to post something", &None);
        assert!(suggestions.contains(&"Post to LinkedIn".to_string()));
        assert!(suggestions.contains(&"Share on Twitter".to_string()));
    }

    #[test]
    fn test_generate_suggestions_platform_specific() {
        let suggestions = generate_suggestions("Help me", &Some("linkedin".to_string()));
        assert!(suggestions.contains(&"Generate professional LinkedIn post".to_string()));
    }

    #[test]
    fn test_generate_suggestions_default() {
        let suggestions = generate_suggestions("Hello", &None);
        assert!(!suggestions.is_empty());
        assert!(suggestions.contains(&"Generate content".to_string()));
    }
}
