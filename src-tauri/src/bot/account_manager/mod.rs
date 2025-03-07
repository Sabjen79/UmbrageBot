use serde::Serialize;
use serenity::all::{CurrentUser, EditProfile};
use tauri::Emitter;

use super::get_http;

#[derive(Serialize, Clone)]
struct BotProfile {
    username: String,
}

pub async fn initialize() {
    // listen("username", async |event, http| {
    //     log_info!("{}", event.payload());
        
    // });
}

pub fn emit_user_update(bot: &CurrentUser) {
    crate::database::bot_accounts::update_account_info(&bot);

    crate::get_app_handle().emit("bot_user_update", BotProfile {
        username: bot.name.clone()
    }).expect("Error emitting user update event");
}

#[tauri::command]
pub async fn change_username(username: String) -> Result<(), String> {
    let http = get_http();

    let result = http
        .get_current_user()
        .await
        .unwrap()
        .edit(http, EditProfile::new().username(username))
        .await;

    match result {
        Ok(()) => {}
        Err(_) => {
            return Err("Changing username too fast!".into())
        }
    }

    Ok(())
}
