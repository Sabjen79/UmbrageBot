use crate::database::{self, activities::DbActivity, bot_accounts::BotAccount};

#[tauri::command]
pub async fn db_validate_token(token: String) -> Result<(), String> {
    database::bot_accounts::validate_token(&token).await
}

#[tauri::command]
pub async fn db_get_all_accounts() -> Result<Vec<BotAccount>, String> {
    database::bot_accounts::get_all()
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
    database::bot_accounts::delete(&id, delete_data)
}

//============================================================================

#[tauri::command]
pub async fn db_get_all_activities() -> Result<Vec<DbActivity>, String> {
    database::activities::get_all().await
}

#[tauri::command]
pub async fn db_insert_activity() -> Result<(), String> {
    database::activities::insert().await
}

#[tauri::command]
pub async fn db_update_activity(activity: DbActivity) -> Result<(), String> {
    database::activities::update(activity).await
}

#[tauri::command]
pub async fn db_delete_activity(id: &str) -> Result<(), String> {
    database::activities::delete(id).await
}