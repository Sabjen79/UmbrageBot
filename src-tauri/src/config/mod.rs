use std::sync::OnceLock;

use crate::logging::info;

static CONFIG_PATH: OnceLock<String> = OnceLock::new();

pub fn initialize(config_path: &str) {
    // Logging handles the path's existence, so the config shouldn't care
    CONFIG_PATH.get_or_init(|| config_path.to_string());

    info!("{}", "Config Initialized");
}

pub fn config_path(next: &str) -> String {
    let path = CONFIG_PATH.get().unwrap().to_string();

    format!("{}{}", path, next)
}
