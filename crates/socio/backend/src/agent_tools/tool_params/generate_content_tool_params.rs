//! Parameters for the GenerateContentTool.
//!
//! This struct defines the parameters required for generating social media content
//! using AI. It includes content specifications, platform requirements, and
//! generation preferences following the project's coding standards.

use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GenerateContentToolRequest {
    /// The topic or subject for the content
    pub topic: String,
    
    /// The social media platform to generate content for
    pub platform: String,
    
    /// The type of content to generate (post, story, caption, etc.)
    pub content_type: String,
    
    /// The desired tone for the content
    pub tone: Option<String>,
    
    /// Target audience for the content
    pub audience: Option<String>,
    
    /// Maximum length for the content
    pub max_length: Option<usize>,
    
    /// Include hashtags in the content
    pub include_hashtags: Option<bool>,
    
    /// Include emojis in the content
    pub include_emojis: Option<bool>,
    
    /// Additional context or requirements
    pub context: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GenerateContentToolResponse {
    /// The generated content
    pub content: String,
    
    /// The platform the content was generated for
    pub platform: String,
    
    /// The content type that was generated
    pub content_type: String,
    
    /// Suggested hashtags (if requested)
    pub hashtags: Option<Vec<String>>,
    
    /// Character count of the generated content
    pub character_count: usize,
    
    /// Whether the content fits platform limits
    pub within_limits: bool,
    
    /// Additional metadata about the generation
    pub metadata: Option<serde_json::Value>,
}

impl Default for GenerateContentToolRequest {
    fn default() -> Self {
        Self {
            topic: "".to_string(),
            platform: "linkedin".to_string(),
            content_type: "post".to_string(),
            tone: Some("professional".to_string()),
            audience: None,
            max_length: Some(280),
            include_hashtags: Some(true),
            include_emojis: Some(true),
            context: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_content_tool_request_default() {
        let request = GenerateContentToolRequest::default();
        assert_eq!(request.platform, "linkedin");
        assert_eq!(request.content_type, "post");
        assert_eq!(request.max_length, Some(280));
        assert_eq!(request.include_hashtags, Some(true));
    }

    #[test]
    fn test_generate_content_tool_request_serialization() {
        let request = GenerateContentToolRequest {
            topic: "AI technology".to_string(),
            platform: "twitter".to_string(),
            content_type: "tweet".to_string(),
            tone: Some("casual".to_string()),
            audience: Some("tech enthusiasts".to_string()),
            max_length: Some(140),
            include_hashtags: Some(true),
            include_emojis: Some(false),
            context: Some("Tech conference announcement".to_string()),
        };

        let json = serde_json::to_string(&request).unwrap();
        let deserialized: GenerateContentToolRequest = serde_json::from_str(&json).unwrap();
        
        assert_eq!(request.topic, deserialized.topic);
        assert_eq!(request.platform, deserialized.platform);
        assert_eq!(request.content_type, deserialized.content_type);
    }

    #[test]
    fn test_generate_content_tool_response_serialization() {
        let response = GenerateContentToolResponse {
            content: "Check out this amazing AI technology!".to_string(),
            platform: "twitter".to_string(),
            content_type: "tweet".to_string(),
            hashtags: Some(vec!["AI".to_string(), "Technology".to_string()]),
            character_count: 45,
            within_limits: true,
            metadata: Some(serde_json::json!({"generated_at": "2024-01-01T00:00:00Z"})),
        };

        let json = serde_json::to_string(&response).unwrap();
        let deserialized: GenerateContentToolResponse = serde_json::from_str(&json).unwrap();
        
        assert_eq!(response.content, deserialized.content);
        assert_eq!(response.platform, deserialized.platform);
        assert_eq!(response.character_count, deserialized.character_count);
    }
}
