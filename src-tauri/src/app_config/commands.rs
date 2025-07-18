use crate::app_config::{self, bot::BotConfig, AppConfiguration};
use tauri::State;

#[tauri::command]
pub async fn get_bot_config(state: State<'_, AppConfiguration>) -> Result<BotConfig, String> {
    Ok(state.bot_config.lock().await.clone())
}

#[tauri::command]
pub async fn set_bot_config(config: BotConfig) -> Result<(), String> {
    app_config::bot::edit_bot_config(|c| {
        config.clone_into(c);
    }).await.map_err(|err| err.to_string())?;
    
    Ok(())
}