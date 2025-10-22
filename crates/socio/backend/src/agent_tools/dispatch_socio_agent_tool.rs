//! Dispatches agent tool requests to appropriate handlers from the agentloop.
//!
//! This function acts as a ToolHandler for the socio backend,
//! routing ToolChoice requests to specific tool implementations within the
//! socio backend. It validates parameters and calls the respective socio tool.
//! Adheres strictly to project Rust coding standards: one item per file, fully qualified paths.

pub async fn dispatch_socio_agent_tool(
    tool_choice: agentloop::types::tool_choice::ToolChoice,
    app_state: actix_web::web::Data<crate::types::app_state::AppState>,
    session_id: agentloop::types::session_id::SessionId,
) -> std::result::Result<
    (agentloop::types::full_tool_response::FullToolResponse, agentloop::types::user_tool_response::UserToolResponse),
    std::string::String,
> {
    // Parse the tool parameters
    let socio_tool_params_result: std::result::Result<
        crate::agent_tools::socio_tool_parameters::SocioToolParameters,
        serde_json::Error,
    > = serde_json::from_value(tool_choice.parameters.clone());

    match socio_tool_params_result {
        std::result::Result::Ok(parsed_tool_params) => {
            // Successfully parsed into a specific tool's parameter structure.
            // Now, dispatch to the correct handler based on the enum variant.
            match parsed_tool_params {
                crate::agent_tools::socio_tool_parameters::SocioToolParameters::GenerateContentTool(params) => {
                    let response = crate::agent_tools::handlers::handle_generate_content_tool::handle_generate_content_tool(
                        params,
                    ).await.map_err(|e| format!("Failed to generate content: {}", e))?;
                    
                    Ok((
                        agentloop::types::full_tool_response::FullToolResponse {
                            tool_name: response.tool_name.clone(),
                            response: response.data,
                        },
                        agentloop::types::user_tool_response::UserToolResponse {
                            tool_name: response.tool_name,
                            summary: response.summary,
                            icon: Some("📝".to_string()),
                            data: Some(response.metadata.unwrap_or(serde_json::Value::Null)),
                        },
                    ))
                }
                crate::agent_tools::socio_tool_parameters::SocioToolParameters::SocialMediaPostTool(params) => {
                    let response = crate::agent_tools::handlers::handle_social_media_post_tool::handle_social_media_post_tool(
                        params,
                    ).await.map_err(|e| format!("Failed to post to social media: {}", e))?;
                    
                    Ok((
                        agentloop::types::full_tool_response::FullToolResponse {
                            tool_name: response.tool_name.clone(),
                            response: response.data,
                        },
                        agentloop::types::user_tool_response::UserToolResponse {
                            tool_name: response.tool_name,
                            summary: response.summary,
                            icon: Some("📱".to_string()),
                            data: Some(response.metadata.unwrap_or(serde_json::Value::Null)),
                        },
                    ))
                }
            }
        }
        std::result::Result::Err(e) => {
            // Failed to deserialize parameters for a known tool.
            std::result::Result::Err(format!(
                "Failed to parse parameters for tool: {}. Parameters (json): {}",
                e,
                serde_json::to_string(&tool_choice.parameters).unwrap_or_else(|_| std::string::String::from("Unserializable parameters"))
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use agentloop::types::tool_choice::ToolChoice;
    use crate::types::app_state::AppState;

    #[tokio::test]
    async fn test_dispatch_generate_content_tool() {
        let tool_choice = ToolChoice {
            parameters: json!{{
                "GenerateContentTool": {
                    "topic": "AI technology",
                    "platform": "linkedin",
                    "content_type": "post",
                    "tone": "professional"
                }
            }},
        };

        // Note: This test would require proper AppState setup
        // For now, we're just testing the parameter parsing
        let socio_tool_params_result: std::result::Result<
            crate::agent_tools::socio_tool_parameters::SocioToolParameters,
            serde_json::Error,
        > = serde_json::from_value(tool_choice.parameters.clone());
        
        assert!(socio_tool_params_result.is_ok());
    }

    #[tokio::test]
    async fn test_dispatch_social_media_post_tool() {
        let tool_choice = ToolChoice {
            parameters: json!{{
                "SocialMediaPostTool": {
                    "platform": "linkedin",
                    "content": "Professional post about AI",
                    "platform_config": {
                        "access_token": "test_token"
                    }
                }
            }},
        };

        let socio_tool_params_result: std::result::Result<
            crate::agent_tools::socio_tool_parameters::SocioToolParameters,
            serde_json::Error,
        > = serde_json::from_value(tool_choice.parameters.clone());
        
        assert!(socio_tool_params_result.is_ok());
    }

    #[tokio::test]
    async fn test_dispatch_invalid_parameters() {
        let tool_choice = ToolChoice {
            parameters: json!{{
                "InvalidTool": {
                    "invalid": "parameters"
                }
            }},
        };

        let socio_tool_params_result: std::result::Result<
            crate::agent_tools::socio_tool_parameters::SocioToolParameters,
            serde_json::Error,
        > = serde_json::from_value(tool_choice.parameters.clone());
        
        assert!(socio_tool_params_result.is_err());
    }
}