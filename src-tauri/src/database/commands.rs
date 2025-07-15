use crate::database::{bot_accounts::BotAccount, Database};

#[tauri::command]
pub async fn db_validate_token(token: String) -> Result<(), String> {
    Database::validate_token(&token).await
}

#[tauri::command]
pub async fn db_get_all_accounts() -> Result<Vec<BotAccount>, String> {
    Database::get_all_accounts().await
}

#[tauri::command]
pub async fn db_insert_account(token: String) -> Result<(), String> {
    Database::insert_account(&token).await
}

#[tauri::command]
pub async fn db_update_account(id: String, new_token: String) -> Result<(), String> {
    Database::update_account_token(&id, &new_token).await
}

#[tauri::command]
pub async fn db_delete_account(id: String, delete_data: bool) -> Result<(), String> {
    Database::delete_account(&id, delete_data).await
}