mod plugins;

use std::sync::Arc;

use smn_web_core::{plugins::plugin_static::PluginStatic, systems::{sys_core::run_server, sys_plugin::PluginManager}};


#[tokio::main]
async fn main() {
    // Create and initialize PluginManager.
    let mut manager = PluginManager::new();
    manager.apply_plugin(Box::new(PluginStatic::new(true)));
    manager.apply_plugin(Box::new(plugins::plugin_ui::PluginUI::new("components".to_string())));
    manager.init_plugins().await;
    let manager = Arc::new(manager);

    // Run the server on port 33311.
    run_server(33311, manager).await;
}
