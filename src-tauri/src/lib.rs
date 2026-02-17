// EW Desktop Agent - Main Library
// This is the core library for the intelligent desktop client

pub mod browser;
pub mod config;
pub mod storage;
pub mod voice;
pub mod ai;

use tracing::info;

/// Initialize the application logging system
pub fn init_logging() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    info!("EW Desktop Agent starting...");
}

/// Get application version
pub fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
