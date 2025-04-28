// src/modules/types.rs

use async_trait::async_trait;
use crate::communicating::types::BotConnection;
use crate::Arc;

#[async_trait]
pub trait C2Module: Send + Sync {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;

    async fn execute(&self, bot: Arc<BotConnection>, args: Vec<String>) -> anyhow::Result<()>;
}
