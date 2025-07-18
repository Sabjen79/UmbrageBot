use crate::database::{self, bot_accounts::BotAccount};

#[tauri::command]
pub async fn db_validate_token(token: String) -> Result<(), String> {
    database::bot_accounts::validate_token(&token).await
}

#[tauri::command]
pub async fn db_get_all_accounts() -> Result<Vec<BotAccount>, String> {
    database::bot_accounts::get_all().await
}

#[tauri::command]
pub async fn db_insert_account(token: String) -> Result<(), String> {
    database::bot_accounts::insert(&token).await
}

#[tauri::command]
pub async fn db_update_account(id: String, new_token: String) -> Result<(), String> {
    database::bot_accounts::update_token(&id, &new_token).await
}

#[tauri::command]
pub async fn db_delete_account(id: String, delete_data: bool) -> Result<(), String> {
    database::bot_accounts::delete(&id, delete_data).await
}