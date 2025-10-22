//! Parameters for the SocialMediaPostTool.
//!
//! This struct defines the parameters required for posting content to social media
//! platforms. It includes platform configuration, content, and posting preferences
//! following the project's coding standards.

use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SocialMediaPostToolRequest {
    /// The social media platform to post to
    pub platform: String,
    
    /// The content to post
    pub content: String,
    
    /// Platform-specific configuration
    pub platform_config: PlatformConfig,
    
    /// Optional image URL to include with the post
    pub image_url: Option<String>,
    
    /// Schedule the post for later (ISO 8601 timestamp)
    pub scheduled_time: Option<String>,
    
    /// Whether to publish immediately or save as draft
    pub publish_immediately: Option<bool>,
    
    /// Additional platform-specific options
    pub platform_options: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PlatformConfig {
    /// API key for the platform
    pub api_key: Option<String>,
    
    /// API secret for the platform
    pub api_secret: Option<String>,
    
    /// Access token for the platform
    pub access_token: Option<String>,
    
    /// Refresh token for the platform
    pub refresh_token: Option<String>,
    
    /// Additional platform-specific configuration
    pub additional_config: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SocialMediaPostToolResponse {
    /// Whether the post was successful
    pub success: bool,
    
    /// The platform the content was posted to
    pub platform: String,
    
    /// The post ID returned by the platform
    pub post_id: Option<String>,
    
    /// The URL of the posted content (if available)
    pub post_url: Option<String>,
    
    /// Any error message if the post failed
    pub error_message: Option<String>,
    
    /// Additional metadata from the platform
    pub metadata: Option<serde_json::Value>,
}

impl Default for SocialMediaPostToolRequest {
    fn default() -> Self {
        Self {
            platform: "linkedin".to_string(),
            content: "".to_string(),
            platform_config: PlatformConfig::default(),
            image_url: None,
            scheduled_time: None,
            publish_immediately: Some(true),
            platform_options: None,
        }
    }
}

impl Default for PlatformConfig {
    fn default() -> Self {
        Self {
            api_key: None,
            api_secret: None,
            access_token: None,
            refresh_token: None,
            additional_config: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_social_media_post_tool_request_default() {
        let request = SocialMediaPostToolRequest::default();
        assert_eq!(request.platform, "linkedin");
        assert_eq!(request.publish_immediately, Some(true));
    }

    #[test]
    fn test_social_media_post_tool_request_serialization() {
        let request = SocialMediaPostToolRequest {
            platform: "twitter".to_string(),
            content: "Hello world!".to_string(),
            platform_config: PlatformConfig {
                api_key: Some("test_key".to_string()),
                api_secret: Some("test_secret".to_string()),
                access_token: None,
                refresh_token: None,
                additional_config: None,
            },
            image_url: Some("https://example.com/image.jpg".to_string()),
            scheduled_time: None,
            publish_immediately: Some(true),
            platform_options: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        let deserialized: SocialMediaPostToolRequest = serde_json::from_str(&json).unwrap();
        
        assert_eq!(request.platform, deserialized.platform);
        assert_eq!(request.content, deserialized.content);
        assert_eq!(request.image_url, deserialized.image_url);
    }

    #[test]
    fn test_social_media_post_tool_response_serialization() {
        let response = SocialMediaPostToolResponse {
            success: true,
            platform: "twitter".to_string(),
            post_id: Some("1234567890".to_string()),
            post_url: Some("https://twitter.com/user/status/1234567890".to_string()),
            error_message: None,
            metadata: Some(serde_json::json!({"likes": 0, "retweets": 0})),
        };

        let json = serde_json::to_string(&response).unwrap();
        let deserialized: SocialMediaPostToolResponse = serde_json::from_str(&json).unwrap();
        
        assert_eq!(response.success, deserialized.success);
        assert_eq!(response.platform, deserialized.platform);
        assert_eq!(response.post_id, deserialized.post_id);
    }
}
