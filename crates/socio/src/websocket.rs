use anyhow::Result;
use actix::prelude::*;
use actix_web_actors::ws;
use serde_json;
use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::{broadcast, RwLock};

use crate::ChatMessage;

#[derive(Debug, Clone)]
pub struct WebSocketManager {
    sessions: Arc<RwLock<HashMap<String, broadcast::Sender<String>>>>,
}

#[derive(Debug)]
pub struct ChatSession {
    pub id: String,
    pub hb: Instant,
    pub ws_manager: Arc<WebSocketManager>,
    pub addr: Option<Addr<ChatSession>>,
}

impl WebSocketManager {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn add_session(&self, session_id: String, tx: broadcast::Sender<String>) {
        let mut sessions = self.sessions.write().await;
        sessions.insert(session_id, tx);
    }

    pub async fn remove_session(&self, session_id: &str) {
        let mut sessions = self.sessions.write().await;
        sessions.remove(session_id);
    }

    pub async fn broadcast_message(&self, session_id: &str, message: &ChatMessage) -> Result<()> {
        let sessions = self.sessions.read().await;
        
        if let Some(tx) = sessions.get(session_id) {
            let message_json = serde_json::to_string(message)?;
            
            // Send to all receivers of this session
            let _ = tx.send(message_json);
        }
        
        Ok(())
    }

    pub async fn broadcast_to_all(&self, message: &ChatMessage) -> Result<()> {
        let sessions = self.sessions.read().await;
        let message_json = serde_json::to_string(message)?;
        
        for tx in sessions.values() {
            let _ = tx.send(message_json.clone());
        }
        
        Ok(())
    }

    pub async fn get_session_count(&self) -> usize {
        let sessions = self.sessions.read().await;
        sessions.len()
    }
}

impl ChatSession {
    pub fn new(session_id: String, ws_manager: Arc<WebSocketManager>) -> Self {
        Self {
            id: session_id,
            hb: Instant::now(),
            ws_manager,
            addr: None,
        }
    }

    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                tracing::warn!("WebSocket Client heartbeat failed, disconnecting!");
                let ws_manager = act.ws_manager.clone();
                let session_id = act.id.clone();
                tokio::spawn(async move {
                    ws_manager.remove_session(&session_id).await;
                });
                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });
    }
}

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

impl Actor for ChatSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
        
        // Create a broadcast channel for this session
        let (tx, mut rx) = broadcast::channel(100);
        
        // Register the session
        let ws_manager = self.ws_manager.clone();
        let session_id = self.id.clone();
        tokio::spawn(async move {
            ws_manager.add_session(session_id.clone(), tx).await;
        });

        // Spawn task to handle incoming messages from the broadcast channel
        let addr = ctx.address();
        tokio::spawn(async move {
            while let Ok(msg) = rx.recv().await {
                if let Err(e) = addr.try_send(Message(msg)) {
                    tracing::error!("Failed to send message to WebSocket: {}", e);
                    break;
                }
            }
        });
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        let ws_manager = self.ws_manager.clone();
        let session_id = self.id.clone();
        tokio::spawn(async move {
            ws_manager.remove_session(&session_id).await;
        });
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

impl Handler<Message> for ChatSession {
    type Result = ();

    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                // Handle incoming text messages
                if let Ok(chat_request) = serde_json::from_str::<crate::ChatRequest>(&text) {
                    // Process the chat request
                    tracing::info!("Received chat request: {}", chat_request.message);
                }
            }
            Ok(ws::Message::Binary(_)) => {
                tracing::warn!("Unexpected binary message received");
            }
            Ok(ws::Message::Continuation(_)) => {
                tracing::warn!("Unexpected continuation message received");
            }
            Ok(ws::Message::Nop) => {
                // No operation message, ignore
            }
            Ok(ws::Message::Close(reason)) => {
                tracing::info!("WebSocket connection closed: {:?}", reason);
                ctx.stop();
            }
            Err(e) => {
                tracing::error!("WebSocket error: {}", e);
                ctx.stop();
            }
        }
    }
}