//! General Agent tool response structure.
//!
//! This module defines the common response structure used by all AI agent tools.
//! It provides a standardized way to return results from tool executions
//! following the project's coding standards.

use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AgentToolResponse {
    /// Whether the tool execution was successful
    pub success: bool,
    
    /// The name of the tool that was executed
    pub tool_name: String,
    
    /// A summary of what was accomplished
    pub summary: String,
    
    /// The main result data from the tool execution
    pub data: serde_json::Value,
    
    /// Any error message if the execution failed
    pub error_message: Option<String>,
    
    /// Additional metadata about the execution
    pub metadata: Option<serde_json::Value>,
    
    /// Timestamp when the tool was executed
    pub executed_at: chrono::DateTime<chrono::Utc>,
}

impl AgentToolResponse {
    /// Creates a successful response.
    ///
    /// # Arguments
    ///
    /// * `tool_name` - The name of the executed tool
    /// * `summary` - Summary of what was accomplished
    /// * `data` - The result data
    ///
    /// # Returns
    ///
    /// A successful AgentToolResponse.
    pub fn success(tool_name: String, summary: String, data: serde_json::Value) -> Self {
        Self {
            success: true,
            tool_name,
            summary,
            data,
            error_message: None,
            metadata: None,
            executed_at: chrono::Utc::now(),
        }
    }
    
    /// Creates a failed response.
    ///
    /// # Arguments
    ///
    /// * `tool_name` - The name of the executed tool
    /// * `error_message` - Description of what went wrong
    ///
    /// # Returns
    ///
    /// A failed AgentToolResponse.
    pub fn error(tool_name: String, error_message: String) -> Self {
        Self {
            success: false,
            tool_name,
            summary: "Tool execution failed".to_string(),
            data: serde_json::Value::Null,
            error_message: Some(error_message),
            metadata: None,
            executed_at: chrono::Utc::now(),
        }
    }
    
    /// Adds metadata to the response.
    ///
    /// # Arguments
    ///
    /// * `metadata` - Additional metadata to include
    ///
    /// # Returns
    ///
    /// Self with the added metadata.
    pub fn with_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_tool_response_success() {
        let response = AgentToolResponse::success(
            "GenerateContentTool".to_string(),
            "Generated LinkedIn post about AI".to_string(),
            serde_json::json!({"content": "AI is transforming industries"}),
        );
        
        assert!(response.success);
        assert_eq!(response.tool_name, "GenerateContentTool");
        assert_eq!(response.summary, "Generated LinkedIn post about AI");
        assert!(response.error_message.is_none());
    }

    #[test]
    fn test_agent_tool_response_error() {
        let response = AgentToolResponse::error(
            "SocialMediaPostTool".to_string(),
            "Failed to authenticate with platform".to_string(),
        );
        
        assert!(!response.success);
        assert_eq!(response.tool_name, "SocialMediaPostTool");
        assert_eq!(response.summary, "Tool execution failed");
        assert!(response.error_message.is_some());
        assert_eq!(response.error_message.unwrap(), "Failed to authenticate with platform");
    }

    #[test]
    fn test_agent_tool_response_with_metadata() {
        let response = AgentToolResponse::success(
            "GenerateContentTool".to_string(),
            "Generated content".to_string(),
            serde_json::json!({"content": "test"}),
        ).with_metadata(serde_json::json!({"character_count": 100}));
        
        assert!(response.metadata.is_some());
        assert_eq!(response.metadata.unwrap()["character_count"], 100);
    }

    #[test]
    fn test_agent_tool_response_serialization() {
        let response = AgentToolResponse::success(
            "TestTool".to_string(),
            "Test summary".to_string(),
            serde_json::json!({"result": "success"}),
        );
        
        let json = serde_json::to_string(&response).unwrap();
        let deserialized: AgentToolResponse = serde_json::from_str(&json).unwrap();
        
        assert_eq!(response.success, deserialized.success);
        assert_eq!(response.tool_name, deserialized.tool_name);
        assert_eq!(response.summary, deserialized.summary);
    }
}
