use serde::{Deserialize, Serialize};
use serenity::all::CurrentUser;
use tauri::Emitter;

pub mod account_username;
pub mod account_status;

// This struct should reflect the store in Svelte
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct BotProfile {
    pub username: String,
}

// pub async fn initialize() {
//     // listen("username", async |event, http| {
//     //     log_info!("{}", event.payload());
        
//     // });
// }

// pub fn emit_user_update(bot: &CurrentUser) {
//     crate::database::bot_accounts::update_account_info(&bot);

//     crate::get_app_handle().emit("bot_user_update", BotProfile {
//         username: bot.name.clone()
//     }).expect("Error emitting user update event");
// }