use std::{error::Error, fs::{self, read_to_string, File}, sync::{LazyLock, OnceLock}};
use serde::{Deserialize, Serialize};
use tauri::Emitter;
use tokio::sync::{RwLock, RwLockReadGuard};
use std::io::prelude::*;
use crate::{logging::log_info, APP_HANDLE};

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

static CONFIG: LazyLock<RwLock<BotConfig>> = LazyLock::new(|| 
    RwLock::new(BotConfig {
        username_timer_enabled: Default::default(),
        username_timer_interval: default_u32::<5>(),
        username_timer_date: Default::default(),
    })
);

static CONFIG_PATH: OnceLock<String> = OnceLock::new();

//==========================================================================//
// Commands for frontend. Since they inevitably clone responses, the backend
// should only call get_config() and edit_config() instead
//==========================================================================//

#[tauri::command]
pub async fn get_bot_config() -> Result<BotConfig, String> {
    Ok(get_config().await.clone())
}

#[tauri::command]
pub async fn set_bot_config(config: BotConfig) -> Result<(), String> {
    edit_config(|c| {
        config.clone_into(c);
    }).await.map_err(|err| err.to_string())?;
    Ok(())
}

//==========================================================================//

pub async fn get_config() -> RwLockReadGuard<'static, BotConfig> {
    CONFIG.read().await
}

pub async fn edit_config<F>(callback: F) -> Result<(), Box<dyn Error>>
where
    F: FnOnce(&mut BotConfig),
{
    callback(&mut *CONFIG.write().await);

    save_config().await?;

    APP_HANDLE.get().unwrap().emit("bot_config_update", get_config().await.clone())?;
    Ok(())
}

pub(super) async fn initialize() -> Result<(), Box<dyn Error>> {
    match crate::bot::ACTIVE_BOT.get() {
        Some(account) => {
            CONFIG_PATH.set(super::config_path(format!("//config//{}.json", account.id).as_str()))?;
        }
        None => {
            return Err("No active bot".into())
        }
    }

    let json;

    match read_to_string(CONFIG_PATH.get().unwrap()) {
        Ok(str) => {
            json = str;

            let config: BotConfig = serde_json::from_str(json.as_str())?;

            *CONFIG.write().await = config;
        }
        Err(_) => {
            log_info!("Bot configuration could not be found! Creating new one");

            let dir_path = super::config_path("//config");

            if !fs::exists(&dir_path)? {
                fs::create_dir(&dir_path)?;
            }

            save_config().await?;

            log_info!("Loaded config");
            
            return Ok(());
        }
    };

    Ok(())
}

async fn save_config() -> Result<(), Box<dyn Error>> {
    let mut file = File::create(CONFIG_PATH.get().unwrap())?;

    let json = serde_json::to_string_pretty(&*CONFIG.read().await)?;
    file.write_all(json.as_bytes())?;

    Ok(())
}