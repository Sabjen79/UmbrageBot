use std::{error::Error, fs::{read_to_string, File}};
use serde::{Deserialize, Serialize};
use serenity::all::OnlineStatus;
use tauri::Emitter;
use std::io::prelude::*;
use crate::{app_handle, app_config::{self, AppConfiguration}, logging::{log_error, log_info}};

// Changes to this type should reflect bot_config.ts
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BotConfig {
    #[serde(default)]
    pub bot_status: OnlineStatus,
}

impl BotConfig {
    pub fn new() -> Self {
        Self {
            bot_status: OnlineStatus::Invisible
        }
    }
}

pub async fn initialize_bot_config(id: String) {
    let app_config = AppConfiguration::get_state();
    let path = format!("{}\\config\\{}.json", app_config.config_path, id);

    let mut path_lock = app_config.bot_config_path.lock().await;
    *path_lock = Some(path.clone());
    drop(path_lock);

    // Avoid holding any lock across await points
    let config: BotConfig = match read_to_string(&path) {
        Ok(str) => {
            match serde_json::from_str(str.as_str()) {
                Ok(cfg) => cfg,
                Err(_) => {
                    log_info!("Bot configuration is invalid! Creating new one");
                    app_config::bot::new_bot_config().await
                }
            }
        }
        Err(_) => {
            log_info!("Bot configuration could not be found! Creating new one");
            app_config::bot::new_bot_config().await
        }
    };

    if let Err(why) = app_handle().emit("bot_config_update", config.clone()) {
        log_error!("Error emitting bot_config_update: {}", why.to_string());
    };

    let mut config_lock = app_config.bot_config.lock().await;
    *config_lock = config;
}

async fn new_bot_config() -> BotConfig {
    let config = BotConfig::new();

    if let Err(why) = app_config::bot::save_bot_config().await {
        log_error!("Cannot save config: {}", why.to_string());
    };

    return config;
}

async fn save_bot_config() -> Result<(), Box<dyn Error>> {
    let app_config = AppConfiguration::get_state();

    let path_opt = {
        let path_lock = app_config.bot_config_path.lock().await;
        path_lock.clone()
    };

    if let Some(path) = path_opt {
        let config_json = {
            let config = app_config.bot_config.lock().await;
            serde_json::to_string_pretty(&*config)?
        };

        let mut file = File::create(path)?;
        file.write_all(config_json.as_bytes())?;
    } else {
        log_error!("Bot Config Path is not initialized");
    }

    Ok(())
}

pub async fn edit_bot_config<F>(callback: F) -> Result<(), Box<dyn Error>>
where
    F: FnOnce(&mut BotConfig),
{
    let app_config = AppConfiguration::get_state();

    {
        let mut config_lock = app_config.bot_config.lock().await;

        callback(&mut *config_lock);

        app_handle().emit("bot_config_update", config_lock.clone())?;
    }

    match app_config::bot::save_bot_config().await {
        Ok(()) => {}
        Err(err) => {
            log_error!("Cannot save config: {}", err.to_string());
        }
    }

    Ok(())
}