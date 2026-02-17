// EW Desktop Agent - Main Entry Point
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use ew_desktop_agent_lib::{init_logging, get_version};

fn main() {
    // Initialize logging
    init_logging();

    // Build and run Tauri application
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            tracing::info!("EW Desktop Agent v{} initialized", get_version());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
