use std::sync::OnceLock;

use crate::logging::{error, info};

pub mod bot_config;

static CONFIG_PATH: OnceLock<String> = OnceLock::new();

pub fn initialize(config_path: &str) {
    // Logging handles the path's existence, so the config shouldn't care
    CONFIG_PATH.set(config_path.to_string()).unwrap();

    info!("{}", "Config Initialized");
}

pub async fn initialize_bot_config() {
    match bot_config::initialize().await {
        Ok(_) => {},
        Err(e) => {
            error!("Error while initializing Config: {}", e);
            return;
        }
    }
}

pub fn config_path(next: &str) -> String {
    let path = CONFIG_PATH.get().unwrap().to_string();

    format!("{}{}", path, next)
}
