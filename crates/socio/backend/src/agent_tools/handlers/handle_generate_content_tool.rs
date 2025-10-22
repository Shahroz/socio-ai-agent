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
    // For now, return a mock response with SEO-friendly content and hashtags
    let (generated_content, seo_tags) = generate_seo_friendly_content(&params);
    
    // Post-process the response based on platform requirements
    let processed_content = post_process_content(&generated_content, &params)?;
    
    // Extract hashtags from content and add SEO tags
    let mut all_hashtags = extract_hashtags(&processed_content).unwrap_or_default();
    all_hashtags.extend(seo_tags);
    all_hashtags.sort();
    all_hashtags.dedup();
    
    let response = GenerateContentToolResponse {
        content: processed_content.clone(),
        platform: params.platform.clone(),
        content_type: params.content_type.clone(),
        hashtags: Some(all_hashtags.clone()),
        character_count: processed_content.len(),
        within_limits: check_platform_limits(&processed_content, &params.platform),
        metadata: Some(serde_json::json!({
            "generated_at": chrono::Utc::now(),
            "tone": params.tone,
            "audience": params.audience,
            "seo_optimized": true,
            "hashtag_count": all_hashtags.len()
        })),
    };
    
    Ok(AgentToolResponse::success(
        "GenerateContentTool".to_string(),
        format!("Generated SEO-friendly {} content for {} about: {} with {} hashtags", 
            params.content_type, params.platform, params.topic, all_hashtags.len()),
        serde_json::to_value(response)?,
    ))
}

/// Generates SEO-friendly content with relevant hashtags.
///
/// # Arguments
///
/// * `params` - The content generation parameters
///
/// # Returns
///
/// A tuple containing the generated content and SEO hashtags.
fn generate_seo_friendly_content(params: &GenerateContentToolRequest) -> (String, Vec<String>) {
    let topic_lower = params.topic.to_lowercase();
    let platform_lower = params.platform.to_lowercase();
    
    // Generate platform-specific content
    let content = match platform_lower.as_str() {
        "linkedin" => generate_linkedin_content(params),
        "twitter" | "x" => generate_twitter_content(params),
        "facebook" => generate_facebook_content(params),
        "instagram" => generate_instagram_content(params),
        _ => generate_generic_content(params),
    };
    
    // Generate SEO-friendly hashtags based on topic and platform
    let seo_tags = generate_seo_hashtags(&topic_lower, &platform_lower, params);
    
    (content, seo_tags)
}

/// Generates LinkedIn-specific content.
fn generate_linkedin_content(params: &GenerateContentToolRequest) -> String {
    let tone = params.tone.as_deref().unwrap_or("professional");
    let audience = params.audience.as_deref().unwrap_or("professionals");
    
    format!(
        "🚀 Exciting developments in {}! This innovative technology is reshaping industries and creating unprecedented opportunities for {}. 

Key insights:
• Transformative impact on business processes
• Enhanced efficiency and productivity
• New career opportunities emerging

What are your thoughts on this evolution? Share your experience in the comments below! 

#{} #{} #Innovation #Technology #Business #Professional #Career #Growth #FutureOfWork",
        params.topic,
        audience,
        params.topic.replace(" ", "").to_uppercase(),
        params.topic.replace(" ", "")
    )
}

/// Generates Twitter-specific content.
fn generate_twitter_content(params: &GenerateContentToolRequest) -> String {
    let tone = params.tone.as_deref().unwrap_or("engaging");
    
    format!(
        "🚀 {} is revolutionizing industries! 

Key points:
✅ Game-changing innovation
✅ Massive market potential  
✅ Future-ready technology

What's your take? 👇

#{} #{} #Innovation #Tech #Future #Disruption",
        params.topic,
        params.topic.replace(" ", "").to_uppercase(),
        params.topic.replace(" ", "")
    )
}

/// Generates Facebook-specific content.
fn generate_facebook_content(params: &GenerateContentToolRequest) -> String {
    let tone = params.tone.as_deref().unwrap_or("friendly");
    let audience = params.audience.as_deref().unwrap_or("community");
    
    format!(
        "🌟 Amazing news about {}! 

This technology is changing the game and creating incredible opportunities for our {}. 

Here's what makes it special:
🔹 Innovative approach to problem-solving
🔹 User-friendly design
🔹 Scalable solutions

Have you experienced this technology? Share your story! 

#{} #{} #Innovation #Technology #Community #Progress #Future",
        params.topic,
        audience,
        params.topic.replace(" ", "").to_uppercase(),
        params.topic.replace(" ", "")
    )
}

/// Generates Instagram-specific content.
fn generate_instagram_content(params: &GenerateContentToolRequest) -> String {
    let tone = params.tone.as_deref().unwrap_or("vibrant");
    
    format!(
        "✨ {} is absolutely game-changing! 

This innovation is:
🎯 Transforming industries
🎯 Creating new possibilities
🎯 Shaping the future

Swipe to see more! 👆

What do you think about this breakthrough? Let us know in the comments! 💬

#{} #{} #Innovation #Technology #Future #Breakthrough #GameChanger #Trending #Viral",
        params.topic,
        params.topic.replace(" ", "").to_uppercase(),
        params.topic.replace(" ", "")
    )
}

/// Generates generic content for unknown platforms.
fn generate_generic_content(params: &GenerateContentToolRequest) -> String {
    format!(
        "🚀 Exciting news about {}! This innovative technology is transforming industries and creating new opportunities. 

Key highlights:
• Revolutionary approach
• Market disruption potential
• Future-ready solutions

#{} #{} #Innovation #Technology #Future",
        params.topic,
        params.topic.replace(" ", "").to_uppercase(),
        params.topic.replace(" ", "")
    )
}

/// Generates SEO-friendly hashtags based on topic and platform.
fn generate_seo_hashtags(topic: &str, platform: &str, params: &GenerateContentToolRequest) -> Vec<String> {
    let mut hashtags = Vec::new();
    
    // Topic-based hashtags
    let topic_tags = match topic {
        t if t.contains("ai") || t.contains("artificial intelligence") => {
            vec!["#AI".to_string(), "#ArtificialIntelligence".to_string(), "#MachineLearning".to_string(), "#TechInnovation".to_string()]
        }
        t if t.contains("blockchain") || t.contains("crypto") => {
            vec!["#Blockchain".to_string(), "#Cryptocurrency".to_string(), "#DeFi".to_string(), "#Web3".to_string()]
        }
        t if t.contains("cloud") => {
            vec!["#CloudComputing".to_string(), "#CloudTech".to_string(), "#DigitalTransformation".to_string(), "#SaaS".to_string()]
        }
        t if t.contains("mobile") || t.contains("app") => {
            vec!["#MobileApp".to_string(), "#MobileTech".to_string(), "#AppDevelopment".to_string(), "#iOS".to_string(), "#Android".to_string()]
        }
        t if t.contains("data") || t.contains("analytics") => {
            vec!["#DataAnalytics".to_string(), "#BigData".to_string(), "#DataScience".to_string(), "#BusinessIntelligence".to_string()]
        }
        t if t.contains("security") || t.contains("cyber") => {
            vec!["#Cybersecurity".to_string(), "#InfoSec".to_string(), "#DataProtection".to_string(), "#Privacy".to_string()]
        }
        _ => {
            vec!["#Technology".to_string(), "#Innovation".to_string(), "#DigitalTransformation".to_string(), "#TechTrends".to_string()]
        }
    };
    
    // Platform-specific hashtags
    let platform_tags = match platform {
        "linkedin" => vec!["#Professional".to_string(), "#Business".to_string(), "#Career".to_string(), "#Leadership".to_string(), "#Networking".to_string()],
        "twitter" | "x" => vec!["#TechTwitter".to_string(), "#TechNews".to_string(), "#Trending".to_string(), "#Viral".to_string(), "#Breaking".to_string()],
        "facebook" => vec!["#Community".to_string(), "#SocialMedia".to_string(), "#Engagement".to_string(), "#Share".to_string(), "#Like".to_string()],
        "instagram" => vec!["#InstaTech".to_string(), "#TechLife".to_string(), "#Innovation".to_string(), "#Creative".to_string(), "#Visual".to_string()],
        _ => vec!["#SocialMedia".to_string(), "#Content".to_string(), "#Engagement".to_string()],
    };
    
    // Audience-specific hashtags
    if let Some(audience) = &params.audience {
        let audience_tags = match audience.to_lowercase().as_str() {
            a if a.contains("developer") || a.contains("engineer") => {
                vec!["#Developers".to_string(), "#Coding".to_string(), "#Programming".to_string(), "#DevLife".to_string()]
            }
            a if a.contains("marketer") || a.contains("marketing") => {
                vec!["#Marketing".to_string(), "#DigitalMarketing".to_string(), "#Growth".to_string(), "#Branding".to_string()]
            }
            a if a.contains("business") || a.contains("entrepreneur") => {
                vec!["#Entrepreneurship".to_string(), "#Startup".to_string(), "#Business".to_string(), "#Leadership".to_string()]
            }
            a if a.contains("student") || a.contains("education") => {
                vec!["#Education".to_string(), "#Learning".to_string(), "#Students".to_string(), "#Academic".to_string()]
            }
            _ => vec!["#Community".to_string(), "#Audience".to_string(), "#Engagement".to_string()],
        };
        hashtags.extend(audience_tags);
    }
    
    // Tone-specific hashtags
    if let Some(tone) = &params.tone {
        let tone_tags = match tone.to_lowercase().as_str() {
            "professional" => vec!["#Professional".to_string(), "#Business".to_string(), "#Corporate".to_string()],
            "casual" => vec!["#Casual".to_string(), "#Friendly".to_string(), "#Relaxed".to_string()],
            "humorous" => vec!["#Funny".to_string(), "#Humor".to_string(), "#Comedy".to_string()],
            "inspiring" => vec!["#Inspiration".to_string(), "#Motivation".to_string(), "#Empowerment".to_string()],
            _ => vec!["#Content".to_string(), "#Engagement".to_string()],
        };
        hashtags.extend(tone_tags);
    }
    
    hashtags.extend(topic_tags);
    hashtags.extend(platform_tags);
    
    // Remove duplicates and limit to reasonable number
    hashtags.sort();
    hashtags.dedup();
    hashtags.truncate(15); // Limit to 15 hashtags max
    
    hashtags
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
