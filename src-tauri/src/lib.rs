// EW Desktop Agent - Main Library
// This is the core library for the intelligent desktop client

pub mod browser;
pub mod config;
pub mod storage;
pub mod voice;
pub mod ai;
pub mod core;
pub mod auth;

use browser::{BrowserState, init_browser_state};
use tracing::info;

/// Initialize the application logging system
pub fn init_logging() {
    let log_dir = dirs::data_local_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("EWDesktopAgent")
        .join("logs");

    // Create log directory if it doesn't exist
    let _ = std::fs::create_dir_all(&log_dir);

    let log_file = log_dir.join("app.log");

    // Setup file logging
    let file_appender = tracing_appender::rolling::daily(&log_dir, "app.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    info!("EW Desktop Agent starting...");
    info!("Log directory: {:?}", log_dir);
}

/// Get application version
pub fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Initialize all Tauri commands and state
pub fn run() {
    info!("Initializing Tauri application...");

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(init_browser_state())
        .invoke_handler(tauri::generate_handler![
            browser::create_browser_tab,
            browser::close_browser_tab,
            browser::set_active_tab,
            browser::get_tabs,
            browser::navigate_tab,
            config::load_business_systems,
            config::save_business_system,
            config::get_app_config,
            storage::init_database,
            storage::save_session,
            storage::load_session,
            storage::save_user_preferences,
            storage::load_user_preferences,
            security::assess_risk,
        ])
        .setup(|app| {
            info!("Application setup complete");

            // Setup system tray
            #[cfg(desktop)]
            {
                use tauri::menu::{Menu, MenuItem};
                use tauri::tray::TrayIconBuilder;

                let quit = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
                let show = MenuItem::with_id(app, "show", "显示窗口", true, None::<&str>)?;
                let menu = Menu::with_items(app, &[&show, &quit])?;

                let _tray = TrayIconBuilder::new()
                    .menu(&menu)
                    .on_menu_event(|app, event| {
                        match event.id.as_ref() {
                            "quit" => {
                                info!("Quit menu clicked");
                                app.exit(0);
                            }
                            "show" => {
                                if let Some(window) = app.get_webview_window("main") {
                                    let _ = window.show();
                                    let _ = window.set_focus();
                                }
                            }
                            _ => {}
                        }
                    })
                    .build(app)?;
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
