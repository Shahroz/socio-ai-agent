use actix_web::{web, HttpRequest, HttpResponse, Result as ActixResult};
use crate::AppState;

/// WebSocket connection endpoint
#[utoipa::path(
    get,
    path = "/ws",
    tag = "websocket",
    responses(
        (status = 101, description = "Switching protocols to WebSocket"),
        (status = 400, description = "Bad request", body = ErrorResponse)
    )
)]
pub async fn handle_websocket(
    req: HttpRequest,
    stream: web::Payload,
    data: web::Data<AppState>,
) -> ActixResult<HttpResponse> {
    let session_id = uuid::Uuid::new_v4().to_string();
    
    actix_web_actors::ws::start(
        crate::websocket::ChatSession::new(session_id, data.ws_manager.clone()),
        &req,
        stream,
    )
}
