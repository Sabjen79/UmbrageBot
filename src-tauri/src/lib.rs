use tauri::Manager;

mod database;
mod config;


// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet, 
            database::bot_accounts::get_all_accounts,
            database::bot_accounts::insert_account,
            database::bot_accounts::update_account_token,
            database::bot_accounts::delete_account
        ])
        .setup(|app| {
            let config_path = format!("{}\\Umbrage Bot", app.path().config_dir().unwrap().to_str().unwrap());

            database::initialize(&config_path);
            config::initialize(&config_path);
            
            Ok(())
         })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    
}
