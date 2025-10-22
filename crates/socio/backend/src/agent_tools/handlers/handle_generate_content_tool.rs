//! Handler for the GenerateContentTool.
//!
//! This handler processes requests to generate social media content using AI.
//! It takes content specifications and generates appropriate content for
//! the specified platform following the project's coding standards.

use anyhow::Result;
use crate::agent_tools::tool_params::generate_content_tool_params::{
    GenerateContentToolRequest, GenerateContentToolResponse
};
use crate::agent_tools::tool_params::agent_tool_response::AgentToolResponse;

/// Handles content generation requests.
///
/// # Arguments
///
/// * `params` - The parameters for content generation
///
/// # Returns
///
/// A `Result` containing the generated content response on success.
pub async fn handle_generate_content_tool(
    params: GenerateContentToolRequest,
) -> Result<AgentToolResponse> {
    let prompt = build_content_generation_prompt(&params);
    
    // TODO: Use the LLM client to generate content
    // For now, return a mock response
    let generated_content = format!(
        "🚀 Exciting news about {}! This innovative technology is transforming industries and creating new opportunities. #{} #Innovation #Technology",
        params.topic,
        params.topic.replace(" ", "").to_uppercase()
    );
    
    // Post-process the response based on platform requirements
    let processed_content = post_process_content(&generated_content, &params)?;
    
    let response = GenerateContentToolResponse {
        content: processed_content.clone(),
        platform: params.platform.clone(),
        content_type: params.content_type.clone(),
        hashtags: extract_hashtags(&processed_content),
        character_count: processed_content.len(),
        within_limits: check_platform_limits(&processed_content, &params.platform),
        metadata: Some(serde_json::json!({
            "generated_at": chrono::Utc::now(),
            "tone": params.tone,
            "audience": params.audience
        })),
    };
    
    Ok(AgentToolResponse::success(
        "GenerateContentTool".to_string(),
        format!("Generated {} content for {} about: {}", 
            params.content_type, params.platform, params.topic),
        serde_json::to_value(response)?,
    ))
}

/// Builds a prompt for content generation based on the parameters.
///
/// # Arguments
///
/// * `params` - The content generation parameters
///
/// # Returns
///
/// A formatted prompt string for the LLM.
fn build_content_generation_prompt(params: &GenerateContentToolRequest) -> String {
    let mut prompt = format!(
        "Generate {} content for {} about '{}'",
        params.content_type, params.platform, params.topic
    );
    
    if let Some(tone) = &params.tone {
        prompt.push_str(&format!(" with a {} tone", tone));
    }
    
    if let Some(audience) = &params.audience {
        prompt.push_str(&format!(" for {}", audience));
    }
    
    if let Some(max_length) = params.max_length {
        prompt.push_str(&format!(" (max {} characters)", max_length));
    }
    
    if params.include_hashtags.unwrap_or(false) {
        prompt.push_str(" and include relevant hashtags");
    }
    
    if params.include_emojis.unwrap_or(false) {
        prompt.push_str(" and include appropriate emojis");
    }
    
    if let Some(context) = &params.context {
        prompt.push_str(&format!(". Additional context: {}", context));
    }
    
    prompt
}

/// Post-processes generated content based on platform requirements.
///
/// # Arguments
///
/// * `content` - The generated content
/// * `params` - The original generation parameters
///
/// # Returns
///
/// Processed content optimized for the platform.
fn post_process_content(content: &str, params: &GenerateContentToolRequest) -> Result<String> {
    let mut processed = content.to_string();
    
    // Apply platform-specific formatting
    match params.platform.to_lowercase().as_str() {
        "twitter" | "x" => {
            // Ensure Twitter character limit
            if processed.len() > 280 {
                processed = processed.chars().take(277).collect();
                processed.push_str("...");
            }
        }
        "linkedin" => {
            // LinkedIn prefers longer, professional content
            if processed.len() < 100 {
                processed.push_str(" What are your thoughts on this topic?");
            }
        }
        "instagram" => {
            // Instagram prefers visual content with emojis
            if !processed.contains("📸") && !processed.contains("✨") {
                processed = format!("✨ {}", processed);
            }
        }
        _ => {}
    }
    
    // Apply length constraints
    if let Some(max_length) = params.max_length {
        if processed.len() > max_length {
            processed = processed.chars().take(max_length - 3).collect();
            processed.push_str("...");
        }
    }
    
    Ok(processed)
}

/// Extracts hashtags from content.
///
/// # Arguments
///
/// * `content` - The content to extract hashtags from
///
/// # Returns
///
/// A vector of hashtags found in the content.
fn extract_hashtags(content: &str) -> Option<Vec<String>> {
    let hashtags: Vec<String> = content
        .split_whitespace()
        .filter(|word| word.starts_with('#'))
        .map(|hashtag| hashtag.to_string())
        .collect();
    
    if hashtags.is_empty() {
        None
    } else {
        Some(hashtags)
    }
}

/// Checks if content fits within platform limits.
///
/// # Arguments
///
/// * `content` - The content to check
/// * `platform` - The target platform
///
/// # Returns
///
/// True if content fits within platform limits.
fn check_platform_limits(content: &str, platform: &str) -> bool {
    match platform.to_lowercase().as_str() {
        "twitter" | "x" => content.len() <= 280,
        "linkedin" => content.len() <= 3000,
        "facebook" => content.len() <= 63206,
        "instagram" => content.len() <= 2200,
        _ => true, // Unknown platform, assume it fits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_handle_generate_content_tool() {
        let params = GenerateContentToolRequest {
            topic: "AI technology".to_string(),
            platform: "linkedin".to_string(),
            content_type: "post".to_string(),
            tone: Some("professional".to_string()),
            audience: Some("tech professionals".to_string()),
            max_length: Some(500),
            include_hashtags: Some(true),
            include_emojis: Some(true),
            context: Some("Tech conference announcement".to_string()),
        };

        let response = handle_generate_content_tool(params).await.unwrap();
        
        assert!(response.success);
        assert_eq!(response.tool_name, "GenerateContentTool");
        assert!(response.summary.contains("Generated post content for linkedin"));
    }

    #[test]
    fn test_build_content_generation_prompt() {
        let params = GenerateContentToolRequest {
            topic: "AI".to_string(),
            platform: "twitter".to_string(),
            content_type: "tweet".to_string(),
            tone: Some("casual".to_string()),
            audience: Some("developers".to_string()),
            max_length: Some(280),
            include_hashtags: Some(true),
            include_emojis: Some(true),
            context: None,
        };

        let prompt = build_content_generation_prompt(&params);
        assert!(prompt.contains("Generate tweet content for twitter about 'AI'"));
        assert!(prompt.contains("casual tone"));
        assert!(prompt.contains("developers"));
        assert!(prompt.contains("max 280 characters"));
    }

    #[test]
    fn test_post_process_content() {
        let params = GenerateContentToolRequest {
            topic: "test".to_string(),
            platform: "twitter".to_string(),
            content_type: "tweet".to_string(),
            tone: None,
            audience: None,
            max_length: Some(100),
            include_hashtags: None,
            include_emojis: None,
            context: None,
        };

        let long_content = "This is a very long content that exceeds the maximum length limit and should be truncated";
        let processed = post_process_content(long_content, &params).unwrap();
        
        assert!(processed.len() <= 100);
        assert!(processed.ends_with("..."));
    }

    #[test]
    fn test_extract_hashtags() {
        let content = "Check out this #AI #technology #innovation";
        let hashtags = extract_hashtags(content).unwrap();
        
        assert_eq!(hashtags.len(), 3);
        assert!(hashtags.contains(&"#AI".to_string()));
        assert!(hashtags.contains(&"#technology".to_string()));
        assert!(hashtags.contains(&"#innovation".to_string()));
    }

    #[test]
    fn test_check_platform_limits() {
        let short_content = "Short content";
        let long_content = "x".repeat(300);
        
        assert!(check_platform_limits(short_content, "twitter"));
        assert!(!check_platform_limits(&long_content, "twitter"));
        assert!(check_platform_limits(&long_content, "linkedin"));
    }
}
