//! Error types for the Socio backend.
//!
//! This module defines all error types used throughout the Socio application.
//! It uses thiserror for proper error handling and follows the project's
//! coding standards with comprehensive error variants.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum SocioError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Social media API error: {0}")]
    SocialMedia(String),
    
    #[error("WebSocket error: {0}")]
    WebSocket(String),
    
    #[error("Authentication error: {0}")]
    Auth(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("Internal server error: {0}")]
    Internal(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    
    #[error("Bad request: {0}")]
    BadRequest(String),
}

impl From<SocioError> for actix_web::Error {
    fn from(err: SocioError) -> Self {
        match err {
            SocioError::Database(_) => actix_web::error::ErrorInternalServerError(err),
            SocioError::SocialMedia(_) => actix_web::error::ErrorBadRequest(err),
            SocioError::WebSocket(_) => actix_web::error::ErrorInternalServerError(err),
            SocioError::Auth(_) => actix_web::error::ErrorUnauthorized(err),
            SocioError::Validation(_) => actix_web::error::ErrorBadRequest(err),
            SocioError::Config(_) => actix_web::error::ErrorInternalServerError(err),
            SocioError::Internal(_) => actix_web::error::ErrorInternalServerError(err),
            SocioError::NotFound(_) => actix_web::error::ErrorNotFound(err),
            SocioError::Unauthorized(_) => actix_web::error::ErrorUnauthorized(err),
            SocioError::BadRequest(_) => actix_web::error::ErrorBadRequest(err),
        }
    }
}

pub type SocioResult<T> = Result<T, SocioError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let error = SocioError::Validation("Invalid input".to_string());
        assert_eq!(error.to_string(), "Validation error: Invalid input");
    }

    #[test]
    fn test_error_from_sqlx() {
        let sqlx_error = sqlx::Error::RowNotFound;
        let socio_error: SocioError = sqlx_error.into();
        assert!(matches!(socio_error, SocioError::Database(_)));
    }
}
