//! OpenAPI specification for the Socio backend.
//!
//! This module defines the OpenAPI specification for all API endpoints
//! in the Socio backend. It uses utoipa for automatic documentation
//! generation following the project's coding standards.

use utoipa::OpenApi;
use crate::routes::chat::{ChatRequest, ChatResponse};
use crate::routes::health::HealthResponse;
use crate::routes::social::{SocialConfigRequest, SocialConfigResponse};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::routes::chat::handle_chat,
        crate::routes::health::health_check,
        crate::routes::social::handle_social_config,
    ),
    components(
        schemas(
            ChatRequest,
            ChatResponse,
            HealthResponse,
            SocialConfigRequest,
            SocialConfigResponse,
        )
    ),
    tags(
        (name = "chat", description = "Chat and conversation endpoints"),
        (name = "health", description = "Health check endpoints"),
        (name = "social", description = "Social media configuration endpoints"),
    ),
    info(
        title = "Socio AI Agent API",
        version = "1.0.0",
        description = "API for the Socio AI Agent - A social media content creation and management platform",
        contact(
            name = "Socio AI Agent Team",
            email = "support@socio-ai-agent.com"
        )
    ),
    servers(
        (url = "http://localhost:3000", description = "Development server"),
        (url = "https://api.socio-ai-agent.com", description = "Production server")
    )
)]
pub struct ApiDoc;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_openapi_spec_generation() {
        let spec = ApiDoc::openapi();
        assert!(!spec.info.title.is_empty());
        assert_eq!(spec.info.title, "Socio AI Agent API");
        assert_eq!(spec.info.version, "1.0.0");
    }
}
