//! Social media configuration API handler for the Socio backend.
//!
//! This handler processes social media platform configuration requests.
//! It manages API credentials and platform settings following the
//! project's coding standards with proper validation and error handling.

use actix_web::{web, HttpResponse, Result as ActixResult};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::types::app_state::AppState;
use crate::services::social_media::{SocialMediaManager, SocialMediaConfig};

#[derive(Debug, Deserialize, ToSchema)]
pub struct SocialConfigRequest {
    pub platform: String,
    pub api_key: Option<String>,
    pub api_secret: Option<String>,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SocialConfigResponse {
    pub success: bool,
    pub message: String,
    pub platform: String,
}

/// Handles social media platform configuration requests.
///
/// # Arguments
///
/// * `req` - The configuration request data
/// * `app_state` - The application state containing social media manager
///
/// # Returns
///
/// An HTTP response containing the configuration result.
#[utoipa::path(
    post,
    path = "/api/social/config",
    request_body = SocialConfigRequest,
    responses(
        (status = 200, description = "Platform configured successfully", body = SocialConfigResponse),
        (status = 400, description = "Bad request", body = SocialConfigResponse),
        (status = 500, description = "Internal server error", body = SocialConfigResponse)
    ),
    tag = "social"
)]
pub async fn handle_social_config(
    req: web::Json<SocialConfigRequest>,
    app_state: web::Data<AppState>,
) -> ActixResult<HttpResponse> {
    // Validate the request
    if let Err(validation_error) = validate_config_request(&req) {
        return Ok(HttpResponse::BadRequest().json(SocialConfigResponse {
            success: false,
            message: validation_error,
            platform: req.platform.clone(),
        }));
    }
    
    // Create the configuration
    let config = SocialMediaConfig {
        platform: req.platform.clone(),
        api_key: req.api_key.clone(),
        api_secret: req.api_secret.clone(),
        access_token: req.access_token.clone(),
        refresh_token: req.refresh_token.clone(),
    };
    
    // Configure the platform
    match configure_platform(config, &app_state.social_manager).await {
        Ok(message) => {
            Ok(HttpResponse::Ok().json(SocialConfigResponse {
                success: true,
                message,
                platform: req.platform.clone(),
            }))
        }
        Err(e) => {
            tracing::error!("Failed to configure platform {}: {}", req.platform, e);
            Ok(HttpResponse::InternalServerError().json(SocialConfigResponse {
                success: false,
                message: format!("Failed to configure platform: {}", e),
                platform: req.platform.clone(),
            }))
        }
    }
}

/// Validates the configuration request.
///
/// # Arguments
///
/// * `req` - The configuration request to validate
///
/// # Returns
///
/// A `Result` indicating validation success or failure.
fn validate_config_request(req: &SocialConfigRequest) -> Result<(), String> {
    if req.platform.trim().is_empty() {
        return Err("Platform name cannot be empty".to_string());
    }
    
    match req.platform.as_str() {
        "linkedin" | "twitter" => {
            if req.api_key.is_none() || req.api_secret.is_none() {
                return Err(format!("{} requires both api_key and api_secret", req.platform));
            }
        }
        "facebook" | "instagram" => {
            if req.access_token.is_none() {
                return Err(format!("{} requires an access_token", req.platform));
            }
        }
        _ => {
            return Err(format!("Unsupported platform: {}", req.platform));
        }
    }
    
    Ok(())
}

/// Configures a social media platform.
///
/// # Arguments
///
/// * `config` - The platform configuration
/// * `social_manager` - The social media manager
///
/// # Returns
///
/// A `Result` containing a success message or error.
async fn configure_platform(
    config: SocialMediaConfig,
    social_manager: &SocialMediaManager,
) -> anyhow::Result<String> {
    // TODO: Implement actual platform configuration
    // For now, just log the configuration
    tracing::info!("Configuring platform: {}", config.platform);
    
    // Simulate async operation
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    
    Ok(format!("Platform '{}' configured successfully", config.platform))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_config_request_linkedin() {
        let req = SocialConfigRequest {
            platform: "linkedin".to_string(),
            api_key: Some("test_key".to_string()),
            api_secret: Some("test_secret".to_string()),
            access_token: None,
            refresh_token: None,
        };
        
        let result = validate_config_request(&req);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_config_request_missing_credentials() {
        let req = SocialConfigRequest {
            platform: "linkedin".to_string(),
            api_key: None,
            api_secret: Some("test_secret".to_string()),
            access_token: None,
            refresh_token: None,
        };
        
        let result = validate_config_request(&req);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("requires both api_key"));
    }

    #[test]
    fn test_validate_config_request_empty_platform() {
        let req = SocialConfigRequest {
            platform: "".to_string(),
            api_key: Some("test_key".to_string()),
            api_secret: Some("test_secret".to_string()),
            access_token: None,
            refresh_token: None,
        };
        
        let result = validate_config_request(&req);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot be empty"));
    }

    #[test]
    fn test_validate_config_request_unsupported_platform() {
        let req = SocialConfigRequest {
            platform: "unsupported".to_string(),
            api_key: Some("test_key".to_string()),
            api_secret: Some("test_secret".to_string()),
            access_token: None,
            refresh_token: None,
        };
        
        let result = validate_config_request(&req);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unsupported platform"));
    }
}
