
use std::sync::Arc;

use plugins::plugin_auth::PluginAuth;
use smn_web_core::{plugins::plugin_static::PluginStatic, systems::{sys_core::run_server, sys_plugin::PluginManager}};

mod plugins;

#[tokio::main]
async fn main() {
    // Create and initialize PluginManager.
    let mut manager = PluginManager::new();

    // Apply the plugins.
    manager.apply_plugin(Box::new(PluginAuth::new()));
    manager.apply_plugin(Box::new(PluginStatic::new(true)));
    
    manager.init_plugins().await;
    let manager = Arc::new(manager);

    // Run the server on port 33311.
    run_server(33311, manager).await;
}