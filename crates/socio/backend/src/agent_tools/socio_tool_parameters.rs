//! Defines an enum to represent all possible Socio tool parameters for direct deserialization.
//!
//! This enum uses `#[serde(untagged)]` to allow `serde_json::from_value` to attempt
//! deserialization of `tool_choice.parameters` into one of its variants based on the
//! distinct structure of the parameter types.
//! Adheres strictly to project Rust coding standards.

use strum_macros::{AsRefStr, EnumIter, EnumProperty, Display};
use schemars::JsonSchema;

/// Enum representing the parameters for various Socio tools.
///
/// Used for deserializing the `parameters` field from an `agentloop::types::tool_choice::ToolChoice`.
/// The `#[serde(untagged)]` attribute means Serde will try to deserialize into each variant
/// in order until one succeeds. This requires the parameter structs to be structurally distinct.
#[derive(serde::Deserialize, std::fmt::Debug, JsonSchema, EnumIter, Display, EnumProperty, AsRefStr)]
#[strum(serialize_all = "snake_case")]
pub enum SocioToolParameters {
    #[schemars(description = "Generates social media content using AI for a specific platform")]
    #[strum(props(description = "Generates social media content using AI for a specific platform"))]
    GenerateContentTool(crate::agent_tools::tool_params::generate_content_tool_params::GenerateContentToolRequest),
    
    #[schemars(description = "Posts content to social media platforms with platform-specific configuration")]
    #[strum(props(description = "Posts content to social media platforms with platform-specific configuration"))]
    SocialMediaPostTool(crate::agent_tools::tool_params::social_media_post_tool_params::SocialMediaPostToolRequest),
}

#[cfg(test)]
mod tests {
    use super::*;
    use schemars::schema_for;
    use serde_json::json;

    #[test]
    fn test_schema() {
        let schema = schema_for!(super::SocioToolParameters);
        // Basic assertion to ensure schema generation runs
        assert!(serde_json::to_string_pretty(&schema).is_ok());
    }
    
    #[test]
    fn test_parsing() {
        let json = json!{{
            "GenerateContentTool": {
                "topic": "AI and technology",
                "platform": "linkedin",
                "content_type": "post",
                "tone": "professional"
            }
        }};

        let socio_tool_params_result: std::result::Result<
            crate::agent_tools::socio_tool_parameters::SocioToolParameters,
            serde_json::Error,
        > = serde_json::from_value(json.clone()); 
        
        assert!(socio_tool_params_result.is_ok());
    }
}
