//! Application state structure for the Socio backend.
//!
//! This struct holds all the shared state needed across the application,
//! including WebSocket manager, social media manager, and LLM client.
//! It follows the project's coding standards with proper documentation.

use std::sync::Arc;
use crate::services::social_media::SocialMediaManager;
use crate::services::websocket::WebSocketManager;

#[derive(Debug, Clone)]
pub struct AppState {
    pub ws_manager: Arc<WebSocketManager>,
    pub social_manager: Arc<SocialMediaManager>,
}

impl AppState {
    /// Creates a new AppState instance with all required components.
    ///
    /// # Returns
    ///
    /// A `Result` containing the new AppState on success, or an error if
    /// any component fails to initialize.
    pub async fn new() -> anyhow::Result<Self> {
        let ws_manager = Arc::new(WebSocketManager::new());
        let social_manager = Arc::new(SocialMediaManager::new());

        Ok(AppState {
            ws_manager,
            social_manager,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_app_state_creation() {
        let app_state = AppState::new().await;
        assert!(app_state.is_ok());
    }
}
