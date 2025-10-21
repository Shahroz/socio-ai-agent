use actix_web::{HttpResponse, Result as ActixResult};
use crate::HealthResponse;
use chrono::Utc;

/// Health check endpoint
#[utoipa::path(
    get,
    path = "/api/health",
    tag = "health",
    responses(
        (status = 200, description = "API is healthy", body = HealthResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn health_check() -> ActixResult<HttpResponse> {
    Ok(HttpResponse::Ok().json(HealthResponse {
        status: "OK".to_string(),
        timestamp: Utc::now(),
        version: "1.0.0".to_string(),
    }))
}
