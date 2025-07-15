use std::{error::Error, fs::{read_to_string, File}};
use serde::{Deserialize, Serialize};
use tauri::Emitter;
use std::io::prelude::*;
use crate::{app_handle, config::{app_config, AppConfiguration}, logging::{log_error, log_info}};

pub const fn default_u32<const V: u32>() -> u32 {
    V
}

// Changes to this type should reflect bot_config.ts
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BotConfig {
    #[serde(default)]
    username_timer_enabled: bool,
    
    #[serde(default = "default_u32::<5>")]
    username_timer_interval: u32,

    #[serde(default)]
    username_timer_date: u32
}

impl BotConfig {
    pub fn new() -> Self {
        Self {
            username_timer_enabled: Default::default(),
            username_timer_interval: default_u32::<5>(),
            username_timer_date: Default::default(),
        }
    }
}

impl AppConfiguration {
    pub async fn initialize_bot_config(id: String) {
        let app_config = app_config();
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
                        AppConfiguration::new_bot_config().await
                    }
                }
            }
            Err(_) => {
                log_info!("Bot configuration could not be found! Creating new one");
                AppConfiguration::new_bot_config().await
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

        if let Err(why) = AppConfiguration::save_bot_config().await {
            log_error!("Cannot save config: {}", why.to_string());
        };

        return config;
    }

    async fn save_bot_config() -> Result<(), Box<dyn Error>> {
        let app_config = app_config();

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
        let app_config = app_config();

        {
            let mut config_lock = app_config.bot_config.lock().await;

            callback(&mut *config_lock);

            app_handle().emit("bot_config_update", config_lock.clone())?;
        }

        match AppConfiguration::save_bot_config().await {
            Ok(()) => {}
            Err(err) => {
                log_error!("Cannot save config: {}", err.to_string());
            }
        }

        Ok(())
    }
}