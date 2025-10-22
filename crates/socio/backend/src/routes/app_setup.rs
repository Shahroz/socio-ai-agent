//! Application setup and configuration for the Socio backend.
//!
//! This module handles the creation and configuration of the Actix-web application
//! with all necessary middleware, routes, and application state. It follows the
//! project's coding standards with proper error handling and documentation.

use anyhow::Result;
use actix_web::{
    web, App, HttpServer, middleware::Logger, middleware::DefaultHeaders,
};
use actix_cors::Cors;
use utoipa_swagger_ui::SwaggerUi;
use utoipa::OpenApi;

use crate::types::app_state::AppState;
use crate::routes::{chat, health, social, websocket};
use crate::openapi::ApiDoc;

/// Creates and configures the Actix-web application.
///
/// # Returns
///
/// A `Result` indicating success or failure of the application setup.
pub async fn create_app() -> Result<()> {
    // Initialize application state
    let app_state = AppState::new().await?;

    // Get server configuration from environment
    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()?;

    tracing::info!("Starting Socio AI Agent server on {}:{}", host, port);

    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(Logger::default())
            .wrap(DefaultHeaders::new().add(("X-Version", "1.0")))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600),
            )
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .service(
                web::scope("/api")
                    .route("/chat", web::post().to(chat::handle_chat))
                    .route("/social/config", web::post().to(social::handle_social_config))
                    .route("/health", web::get().to(health::health_check))
                    .route("/openapi.json", web::get().to(serve_openapi)),
            )
            .route("/ws", web::get().to(websocket::handle_websocket))
            .service(
                actix_files::Files::new("/", "./crates/socio/frontend/dist")
                    .index_file("index.html"),
            )
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await?;

    Ok(())
}

/// Serves the OpenAPI specification.
///
/// # Returns
///
/// An HTTP response containing the OpenAPI JSON specification.
async fn serve_openapi() -> actix_web::Result<actix_web::HttpResponse> {
    Ok(actix_web::HttpResponse::Ok().json(ApiDoc::openapi()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_app_state_creation() {
        // This test will only pass if GEMINI_API_KEY is set
        if std::env::var("GEMINI_API_KEY").is_ok() {
            let app_state = AppState::new().await;
            assert!(app_state.is_ok());
        } else {
            println!("Skipping AppState test - GEMINI_API_KEY not set");
        }
    }
}
