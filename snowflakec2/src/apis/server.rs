use axum::{
    Router,
    routing::post,
    extract::{Extension, Json},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use anyhow::Result;
use crate::core::state::ServerState;
use axum::serve;
use tokio::net::TcpListener;

#[derive(Deserialize)]
struct CommandRequest {
    session_token: String,
    bot_id: String,
    command: String,
    args: Vec<String>,
}

#[derive(Serialize)]
struct BotInfo {
    bot_id: String,
}

#[derive(Serialize)]
struct ModuleInfo {
    name: String,
    description: String,
}

// Handler: list bots
async fn list_bots(
    Extension(state): Extension<Arc<ServerState>>,
) -> Result<Json<Vec<BotInfo>>, StatusCode> {
    let bots = state.bots.lock().await;
    let mut bot_list = Vec::new();

    for (bot_id, _bot_conn) in bots.iter() {
        bot_list.push(BotInfo {
            bot_id: bot_id.to_string(),
        });
    }

    Ok(Json(bot_list))
}

// Handler: list modules
async fn list_modules(
    Extension(state): Extension<Arc<ServerState>>,
) -> Result<Json<Vec<ModuleInfo>>, StatusCode> {
    let modules = state.modules.lock().await;
    let mut module_list = Vec::new();

    for (name, module) in modules.iter() {
        module_list.push(ModuleInfo {
            name: name.clone(),
            description: module.description().to_string(), // Assuming Module struct has description field
        });
    }

    Ok(Json(module_list))
}

// Handler: send command
async fn send_command(
    Extension(state): Extension<Arc<ServerState>>,
    Json(req): Json<CommandRequest>,
) -> Result<StatusCode, StatusCode> {
    let bot_id = match uuid::Uuid::parse_str(&req.bot_id) {
        Ok(id) => id,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };

    // Validate session
    let sessions = state.sessions.lock().await;
    let _session = match sessions.get(&req.session_token) {
        Some(sess) => sess,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    // Validate bot exists
    let bots = state.bots.lock().await;
    let bot = match bots.get(&bot_id) {
        Some(bot) => bot,
        None => return Err(StatusCode::NOT_FOUND),
    };

    // Validate module exists
    let modules = state.modules.lock().await;
    let module = match modules.get(&req.command) {
        Some(module) => module.clone(),
        None => return Err(StatusCode::BAD_REQUEST),
    };
    drop(modules); // unlock early
    drop(sessions); // unlock early

    // Execute module
    module.execute(Arc::new(bot.clone()), req.args).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}

// Function: start API server
pub async fn start_api_server(state: Arc<ServerState>) -> Result<()> {
    let app = Router::new()
        .route("/send_command", post(send_command))
        .route("/list_bots", post(list_bots))
        .route("/modules", post(list_modules))
        .layer(Extension(state));

    println!("[*] API Server started on 0.0.0.0:8000...");

    // New: Use tokio TcpListener
    let listener = TcpListener::bind("0.0.0.0:8000").await?;

    // New: Use axum::serve instead of axum::Server::bind
    serve(listener, app).await?;

    Ok(())
}
