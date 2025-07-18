use serenity::all::OnlineStatus;

use crate::bot;

#[tauri::command]
pub async fn bot_set_username(username: String) -> Result<(), String> {
    bot::account_manager::set_username(username.as_str())
        .await.map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn bot_set_status(status: u8) -> Result<(), String> {
    let status = match status {
        0 => OnlineStatus::Online,
        1 => OnlineStatus::Idle,
        2 => OnlineStatus::DoNotDisturb,
        3 => OnlineStatus::Invisible,
        _ => return Err("Invalid status number".into())
    };

    bot::account_manager::set_status(status)
        .await.map_err(|err| err.to_string())
}