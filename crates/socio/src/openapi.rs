use utoipa::OpenApi;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::chat::handle_chat,
        crate::api::social::handle_social_config,
        crate::api::health::health_check,
        crate::api::websocket::handle_websocket
    ),
    components(
        schemas(
            ChatRequest,
            ChatResponse,
            ChatMessage,
            SocialMediaConfig,
            ErrorResponse,
            HealthResponse,
            ConfigResponse
        )
    ),
    tags(
        (name = "chat", description = "Chat and messaging endpoints"),
        (name = "social", description = "Social media platform configuration"),
        (name = "websocket", description = "WebSocket real-time communication"),
        (name = "health", description = "Health check endpoints")
    ),
    info(
        title = "Socio AI Agent API",
        description = "AI Agent for Social Media Content Management",
        version = "1.0.0",
        contact(
            name = "Shahroz Alauddin",
            email = "shahroz@example.com"
        )
    ),
    servers(
        (url = "http://localhost:3000", description = "Development server")
    )
)]
pub struct ApiDoc;

/// Chat request payload
#[derive(Serialize, Deserialize, utoipa::ToSchema)]
pub struct ChatRequest {
    /// The message content
    #[schema(example = "Generate a LinkedIn post about AI trends")]
    pub message: String,
    
    /// Optional session ID for conversation continuity
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub session_id: Option<String>,
}

/// Chat response payload
#[derive(Serialize, Deserialize, utoipa::ToSchema)]
pub struct ChatResponse {
    /// The response message
    pub message: ChatMessage,
    
    /// Session ID for the conversation
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub session_id: String,
}

/// Individual chat message
#[derive(Serialize, Deserialize, utoipa::ToSchema)]
pub struct ChatMessage {
    /// Unique message identifier
    #[schema(example = "msg_123")]
    pub id: String,
    
    /// Message content
    #[schema(example = "Here's a LinkedIn post about AI trends...")]
    pub content: String,
    
    /// Message timestamp
    #[schema(value_type = String, format = DateTime)]
    pub timestamp: DateTime<Utc>,
    
    /// Message sender
    #[schema(example = "assistant")]
    pub sender: String,
}

/// Social media platform configuration
#[derive(Serialize, Deserialize, utoipa::ToSchema)]
pub struct SocialMediaConfig {
    /// Social media platform
    #[schema(example = "linkedin")]
    pub platform: String,
    
    /// API key for the platform
    #[schema(example = "your_api_key")]
    pub api_key: Option<String>,
    
    /// API secret for the platform
    #[schema(example = "your_api_secret")]
    pub api_secret: Option<String>,
    
    /// Access token for the platform
    #[schema(example = "your_access_token")]
    pub access_token: Option<String>,
}

/// Error response
#[derive(Serialize, Deserialize, utoipa::ToSchema)]
pub struct ErrorResponse {
    /// Error type
    #[schema(example = "validation_error")]
    pub error: String,
    
    /// Error message
    #[schema(example = "Invalid request parameters")]
    pub message: String,
}

/// Health check response
#[derive(Serialize, Deserialize, utoipa::ToSchema)]
pub struct HealthResponse {
    /// Health status
    #[schema(example = "OK")]
    pub status: String,
    
    /// Server timestamp
    #[schema(value_type = String, format = DateTime)]
    pub timestamp: DateTime<Utc>,
    
    /// API version
    #[schema(example = "1.0.0")]
    pub version: String,
}

/// Success response for configuration
#[derive(Serialize, Deserialize, utoipa::ToSchema)]
pub struct ConfigResponse {
    /// Success status
    #[schema(example = true)]
    pub success: bool,
    
    /// Response message
    #[schema(example = "Platform configured successfully")]
    pub message: String,
}
