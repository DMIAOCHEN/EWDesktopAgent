// EW Desktop Agent - Main Entry Point
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use ew_desktop_agent_lib::init_logging;

fn main() {
    // Initialize logging
    init_logging();

    // Run the Tauri application
    ew_desktop_agent_lib::run();
}
