use actix_web::{web, HttpResponse, Result as ActixResult};
use crate::{AppState, SocialMediaConfig, ConfigResponse, ErrorResponse};

/// Configure social media platform API credentials
#[utoipa::path(
    post,
    path = "/api/social/config",
    tag = "social",
    request_body = SocialMediaConfig,
    responses(
        (status = 200, description = "Configuration successful", body = ConfigResponse),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn handle_social_config(
    config: web::Json<SocialMediaConfig>,
    data: web::Data<AppState>,
) -> ActixResult<HttpResponse> {
    match data.social_manager.configure_platform(&config).await {
        Ok(_) => Ok(HttpResponse::Ok().json(ConfigResponse {
            success: true,
            message: format!("{} configured successfully", config.platform),
        })),
        Err(e) => {
            tracing::error!("Failed to configure social media platform: {}", e);
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                error: "configuration_error".to_string(),
                message: format!("Failed to configure platform: {}", e),
            }))
        }
    }
}
