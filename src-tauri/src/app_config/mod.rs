use std::{fs, io};

use tauri::{Manager, State};
use tokio::sync::Mutex;

use crate::{app_handle, app_config::bot::BotConfig, logging::{log_error, log_info}};

pub mod bot;
pub mod commands;

/// Handles all operations regarding the `config` folder in `AppData`
pub struct AppConfiguration {
    config_path: String,
    bot_config_path: Mutex<Option<String>>,
    bot_config: Mutex<BotConfig>
}

impl AppConfiguration {
    pub fn new(config_path: &str) -> AppConfiguration {
        log_info!("{}", "Config Initialized");

        match fs::create_dir(format!("{}\\config", config_path)) {
            Ok(()) => {},
            Err(e) => {
                if e.kind() != io::ErrorKind::AlreadyExists {
                    log_error!("Error creating config directory: {}", e.to_string())
                }
            }
        }

        Self { 
            config_path: config_path.to_owned(),
            bot_config_path: Mutex::new(Option::None),
            bot_config: Mutex::new(BotConfig::new())
        }
    }

    pub(in crate::app_config)
    fn get_state() -> State<'static, AppConfiguration> {
        app_handle().state::<AppConfiguration>()
    }
}

pub fn config_path() -> String {
    let state = AppConfiguration::get_state();

    return state.config_path.clone();
}