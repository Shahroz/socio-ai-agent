use anyhow::Result;
use actix_web::{
    web, App, HttpServer, HttpResponse, Result as ActixResult,
    middleware::Logger, middleware::DefaultHeaders,
};
use actix_cors::Cors;
use std::sync::Arc;
use llm::GeminiClient;
use utoipa_swagger_ui::SwaggerUi;
use utoipa::OpenApi;

pub mod social_media;
pub mod websocket;
pub mod openapi;
pub mod api;

use social_media::SocialMediaManager;
use websocket::WebSocketManager;
use openapi::{ApiDoc, ChatRequest, ChatResponse, ChatMessage, SocialMediaConfig, ErrorResponse, HealthResponse, ConfigResponse};
use api::{chat, social, health, websocket as ws_api};

#[derive(Debug, Clone)]
pub struct AppState {
    pub ws_manager: Arc<WebSocketManager>,
    pub social_manager: Arc<SocialMediaManager>,
    pub llm_client: Arc<GeminiClient>,
}

pub async fn create_app() -> Result<()> {
    let ws_manager = Arc::new(WebSocketManager::new());
    let social_manager = Arc::new(SocialMediaManager::new());
    let llm_client = Arc::new(GeminiClient::new()?);

    let app_state = AppState {
        ws_manager,
        social_manager,
        llm_client,
    };

    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()?;

    tracing::info!("Starting Socio AI Agent server on {}:{}", host, port);

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
            .route("/ws", web::get().to(ws_api::handle_websocket))
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

async fn serve_openapi() -> ActixResult<HttpResponse> {
    Ok(HttpResponse::Ok().json(ApiDoc::openapi()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_gemini_client_creation() {
        // This test will only pass if GEMINI_API_KEY is set
        if std::env::var("GEMINI_API_KEY").is_ok() {
            let client = GeminiClient::new();
            assert!(client.is_ok());
        } else {
            println!("Skipping Gemini test - API key not set");
        }
    }

    #[test]
    fn test_chat_message_serialization() {
        let message = ChatMessage {
            id: "test_id".to_string(),
            content: "Test message".to_string(),
            timestamp: chrono::Utc::now(),
            sender: "user".to_string(),
        };

        let json = serde_json::to_string(&message).unwrap();
        let deserialized: ChatMessage = serde_json::from_str(&json).unwrap();
        
        assert_eq!(message.id, deserialized.id);
        assert_eq!(message.content, deserialized.content);
        assert_eq!(message.sender, deserialized.sender);
    }

    #[test]
    fn test_social_media_config() {
        let config = SocialMediaConfig {
            platform: "linkedin".to_string(),
            api_key: Some("test_key".to_string()),
            api_secret: Some("test_secret".to_string()),
            access_token: None,
        };

        assert_eq!(config.platform, "linkedin");
        assert!(config.api_key.is_some());
        assert!(config.access_token.is_none());
    }
}