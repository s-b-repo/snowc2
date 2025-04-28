// src/core/state.rs

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::modules::types::C2Module;
use crate::communicating::types::BotConnection;

pub struct ServerState {
    pub bots: Arc<Mutex<HashMap<Uuid, BotConnection>>>,
    pub modules: Arc<Mutex<HashMap<String, Arc<dyn C2Module + Send + Sync>>>>,
    pub sessions: Arc<Mutex<HashMap<String, SessionInfo>>>,
}

pub struct SessionInfo {
    pub username: String,
    pub is_admin: bool,
}

impl ServerState {
    pub fn new() -> Self {
        ServerState {
            bots: Arc::new(Mutex::new(HashMap::new())),
            modules: Arc::new(Mutex::new(HashMap::new())),
            sessions: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}
