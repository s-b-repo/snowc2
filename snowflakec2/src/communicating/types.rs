// src/communicating/types.rs

use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct BotConnection {
    pub connection: Arc<Mutex<TcpStream>>,
}
impl BotConnection {
    pub async fn send_message(&self, message: &str) -> anyhow::Result<()> {
        let mut conn = self.connection.lock().await;
        conn.write_all(format!("{}\n", message).as_bytes()).await?;
        Ok(())
    }

    pub async fn read_message(&self) -> anyhow::Result<String> {
        let mut buffer = vec![0u8; 1024];
        let mut conn = self.connection.lock().await;
        let n = conn.read(&mut buffer).await?;
        Ok(String::from_utf8_lossy(&buffer[..n]).to_string())
    }
}
