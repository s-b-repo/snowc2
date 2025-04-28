// src/modules/loader.rs


use std::sync::Arc;
use anyhow::Result;
use crate::core::state::ServerState;
use crate::modules::types::C2Module;
use crate::modules::ping::PingModule;
use crate::modules::download::DownloadModule;
use crate::modules::execute::ExecuteModule;
// List your modules here manually for now (dynamic detection later if needed)

pub async fn load_modules(state: &Arc<ServerState>) -> Result<()> {
    let mut modules = state.modules.lock().await;

    let built_in_modules: Vec<Arc<dyn C2Module + Send + Sync>> = vec![
        Arc::new(PingModule {}),
        Arc::new(DownloadModule::new()),
        Arc::new(ExecuteModule::new()),
    ];

    for module in built_in_modules {
        println!("[+] Loaded module: {}", module.name());
        modules.insert(module.name().to_string(), module);
    }

    Ok(())
}
