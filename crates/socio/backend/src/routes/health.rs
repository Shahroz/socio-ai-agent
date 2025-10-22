//! Health check API handler for the Socio backend.
//!
//! This handler provides health check endpoints to monitor the application
//! status. It follows the project's coding standards with proper response
//! formatting and status reporting.

use actix_web::{HttpResponse, Result as ActixResult};
use serde::Serialize;
use utoipa::ToSchema;
use chrono::Utc;

#[derive(Debug, Serialize, ToSchema)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub version: String,
    pub uptime: u64,
}

/// Handles health check requests.
///
/// # Returns
///
/// An HTTP response containing the health status.
#[utoipa::path(
    get,
    path = "/api/health",
    responses(
        (status = 200, description = "Service is healthy", body = HealthResponse),
        (status = 500, description = "Internal server error", body = String)
    ),
    tag = "health"
)]
pub async fn health_check() -> ActixResult<HttpResponse> {
    let health_response = HealthResponse {
        status: "healthy".to_string(),
        timestamp: Utc::now(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime: get_uptime(),
    };
    
    Ok(HttpResponse::Ok().json(health_response))
}

/// Gets the application uptime in seconds.
///
/// # Returns
///
/// The uptime in seconds since application start.
fn get_uptime() -> u64 {
    // TODO: Implement proper uptime tracking
    // For now, return a mock value
    3600 // 1 hour in seconds
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_check() {
        let response = health_check().await;
        assert!(response.is_ok());
        
        let http_response = response.unwrap();
        assert_eq!(http_response.status(), actix_web::http::StatusCode::OK);
    }

    #[test]
    fn test_get_uptime() {
        let uptime = get_uptime();
        assert!(uptime > 0);
    }
}
