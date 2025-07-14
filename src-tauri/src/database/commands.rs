use tauri::State;

use crate::database::{bot_accounts::BotAccount, Database};


#[tauri::command]
pub async fn db_validate_token(db: State<'_, Database>, token: String) -> Result<(), String> {
    db.validate_token(&token).await
}

#[tauri::command]
pub async fn db_get_all_accounts(db: State<'_, Database>) -> Result<Vec<BotAccount>, String> {
    db.get_all_accounts().await
}

#[tauri::command]
pub async fn db_insert_account(db: State<'_, Database>, token: String) -> Result<(), String> {
    db.insert_account(&token).await
}

#[tauri::command]
pub async fn db_update_account(db: State<'_, Database>, id: String, new_token: String) -> Result<(), String> {
    db.update_account_token(&id, &new_token).await
}

#[tauri::command]
pub async fn db_delete_account(db: State<'_, Database>, id: String, delete_data: bool) -> Result<(), String> {
    db.delete_account(&id, delete_data).await
}