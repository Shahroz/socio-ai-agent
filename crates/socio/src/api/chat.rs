use actix_web::{web, HttpResponse, Result as ActixResult};
use crate::{AppState, ChatRequest, ChatResponse, ChatMessage};
use uuid::Uuid;
use chrono::Utc;

/// Send a chat message to the AI agent
#[utoipa::path(
    post,
    path = "/api/chat",
    tag = "chat",
    request_body = ChatRequest,
    responses(
        (status = 200, description = "Successful response", body = ChatResponse),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn handle_chat(
    request: web::Json<ChatRequest>,
    data: web::Data<AppState>,
) -> ActixResult<HttpResponse> {
    let session_id = request.session_id.clone().unwrap_or_else(|| Uuid::new_v4().to_string());
    
    let _user_message = ChatMessage {
        id: Uuid::new_v4().to_string(),
        content: request.message.clone(),
        timestamp: Utc::now(),
        sender: "user".to_string(),
    };

    // Process the message and generate response
    let response_content = match process_message(&request.message, &data).await {
        Ok(content) => content,
        Err(e) => format!("Error processing message: {}", e),
    };

    let assistant_message = ChatMessage {
        id: Uuid::new_v4().to_string(),
        content: response_content,
        timestamp: Utc::now(),
        sender: "assistant".to_string(),
    };

    // Broadcast to WebSocket clients
    if let Err(e) = data.ws_manager.broadcast_message(&session_id, &assistant_message).await {
        tracing::error!("Failed to broadcast message: {}", e);
    }

    Ok(HttpResponse::Ok().json(ChatResponse {
        message: assistant_message,
        session_id,
    }))
}

async fn process_message(message: &str, state: &AppState) -> anyhow::Result<String> {
    // Use Gemini to process the message and generate appropriate response
    let prompt = format!(
        "You are a professional social media content manager AI assistant. \
        The user has sent you this message: '{}' \
        Please provide a helpful, professional response that addresses their request. \
        If they're asking for content generation, provide specific suggestions. \
        If they're asking for help, provide clear guidance. \
        Keep responses concise but informative.",
        message
    );

    state.llm_client.generate_content(&prompt, Some(0.7)).await
}
