pub mod commands;
pub mod config_composer;
pub mod errors;
pub mod models;
pub mod ssh_client;
pub mod subscription_parser;

#[cfg(test)]
mod subscription_parser_tests;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::connect_and_fetch_config,
            commands::fetch_subscription,
            commands::apply_remote_config,
            commands::rollback_remote_config,
            commands::test_proxy_connection,
            commands::switch_active_outbound,
            commands::update_inbound_port
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
