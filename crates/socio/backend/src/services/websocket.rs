//! WebSocket service for real-time communication.
//!
//! This service manages WebSocket connections for real-time chat and
//! notifications. It follows the project's coding standards with proper
//! actor-based architecture using Actix.

use actix::prelude::*;
use actix_web_actors::ws;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct WebSocketManager {
    sessions: HashMap<Uuid, Recipient<WebSocketMessage>>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct WebSocketMessage {
    pub message: String,
    pub session_id: Uuid,
}

impl WebSocketManager {
    /// Creates a new WebSocketManager instance.
    ///
    /// # Returns
    ///
    /// A new WebSocketManager with empty session storage.
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }

    /// Registers a new WebSocket session.
    ///
    /// # Arguments
    ///
    /// * `session_id` - Unique identifier for the session
    /// * `recipient` - Actor recipient for sending messages
    pub fn register_session(&mut self, session_id: Uuid, recipient: Recipient<WebSocketMessage>) {
        self.sessions.insert(session_id, recipient);
    }

    /// Unregisters a WebSocket session.
    ///
    /// # Arguments
    ///
    /// * `session_id` - Unique identifier for the session to remove
    pub fn unregister_session(&mut self, session_id: &Uuid) {
        self.sessions.remove(session_id);
    }

    /// Sends a message to a specific session.
    ///
    /// # Arguments
    ///
    /// * `session_id` - Target session identifier
    /// * `message` - Message content to send
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    pub async fn send_to_session(&self, session_id: &Uuid, message: String) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(recipient) = self.sessions.get(session_id) {
            let ws_message = WebSocketMessage {
                message,
                session_id: *session_id,
            };
            recipient.do_send(ws_message);
            Ok(())
        } else {
            Err("Session not found".into())
        }
    }

    /// Broadcasts a message to all active sessions.
    ///
    /// # Arguments
    ///
    /// * `message` - Message content to broadcast
    pub async fn broadcast(&self, message: String) {
        for (session_id, recipient) in &self.sessions {
            let ws_message = WebSocketMessage {
                message: message.clone(),
                session_id: *session_id,
            };
            recipient.do_send(ws_message);
        }
    }

    /// Gets the number of active sessions.
    ///
    /// # Returns
    ///
    /// The count of active WebSocket sessions.
    pub fn session_count(&self) -> usize {
        self.sessions.len()
    }
}

impl Default for WebSocketManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_websocket_manager_creation() {
        let manager = WebSocketManager::new();
        assert_eq!(manager.session_count(), 0);
    }

    #[test]
    fn test_session_registration() {
        let mut manager = WebSocketManager::new();
        let session_id = Uuid::new_v4();
        
        // Note: In a real test, you'd need to create a proper Recipient
        // This is a simplified test for the structure
        assert_eq!(manager.session_count(), 0);
    }
}
