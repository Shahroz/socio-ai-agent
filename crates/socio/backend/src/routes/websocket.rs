//! WebSocket API handler for the Socio backend.
//!
//! This handler manages WebSocket connections for real-time communication.
//! It uses Actix actors for WebSocket management following the project's
//! coding standards with proper error handling and connection management.

use actix_web::{web, HttpRequest, HttpResponse, Result as ActixResult};
use actix_web_actors::ws;
use actix::ActorContext;
use uuid::Uuid;

/// Handles WebSocket connection requests.
///
/// # Arguments
///
/// * `req` - The HTTP request
/// * `stream` - The WebSocket stream
///
/// # Returns
///
/// An HTTP response initiating the WebSocket connection.
pub async fn handle_websocket(
    req: HttpRequest,
    stream: web::Payload,
) -> ActixResult<HttpResponse> {
    let session_id = Uuid::new_v4();
    
    tracing::info!("New WebSocket connection with session ID: {}", session_id);
    
    // Create WebSocket actor
    let ws_actor = WebSocketActor::new(session_id);
    
    // Start the WebSocket connection
    let resp = ws::start(ws_actor, &req, stream)?;
    
    Ok(resp)
}

/// WebSocket actor for handling real-time communication.
pub struct WebSocketActor {
    pub session_id: Uuid,
}

impl WebSocketActor {
    /// Creates a new WebSocket actor.
    ///
    /// # Arguments
    ///
    /// * `session_id` - Unique identifier for the WebSocket session
    ///
    /// # Returns
    ///
    /// A new WebSocketActor instance.
    pub fn new(session_id: Uuid) -> Self {
        Self { session_id }
    }
}

impl actix::Actor for WebSocketActor {
    type Context = ws::WebsocketContext<Self>;
    
    /// Called when the actor starts.
    fn started(&mut self, ctx: &mut Self::Context) {
        tracing::info!("WebSocket actor started for session: {}", self.session_id);
        
        // Send welcome message
        ctx.text("Welcome to Socio AI Agent! You are now connected.");
    }
    
    /// Called when the actor stops.
    fn stopped(&mut self, _ctx: &mut Self::Context) {
        tracing::info!("WebSocket actor stopped for session: {}", self.session_id);
    }
}

impl actix::StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketActor {
    /// Handles incoming WebSocket messages.
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                // Handle pong message
            }
            Ok(ws::Message::Text(text)) => {
                tracing::info!("Received text message from session {}: {}", self.session_id, text);
                
                // Echo the message back with a prefix
                let response = format!("Echo: {}", text);
                ctx.text(response);
            }
            Ok(ws::Message::Binary(bin)) => {
                tracing::info!("Received binary message from session {}: {} bytes", self.session_id, bin.len());
                
                // Echo binary message back
                ctx.binary(bin);
            }
            Ok(ws::Message::Close(reason)) => {
                tracing::info!("WebSocket connection closed for session {}: {:?}", self.session_id, reason);
                ctx.stop();
            }
            Ok(ws::Message::Continuation(_)) | Ok(ws::Message::Nop) => {
                // Ignore continuation and nop messages
            }
            Err(e) => {
                tracing::error!("WebSocket error for session {}: {:?}", self.session_id, e);
                ctx.stop();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_websocket_actor_creation() {
        let session_id = Uuid::new_v4();
        let actor = WebSocketActor::new(session_id);
        assert_eq!(actor.session_id, session_id);
    }
}
