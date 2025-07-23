use crate::app_config::{self, bot::BotConfig, AppConfiguration};
use tauri::State;

#[tauri::command]
pub fn get_bot_config(state: State<'_, AppConfiguration>) -> Result<BotConfig, String> {
    Ok(state.bot_config.lock().unwrap().clone())
}

#[tauri::command]
pub fn set_bot_config(config: BotConfig) -> Result<(), String> {
    app_config::bot::edit_bot_config_frontend(|c| {
        config.clone_into(c);
    }).map_err(|err| err.to_string())?;
    
    Ok(())
}