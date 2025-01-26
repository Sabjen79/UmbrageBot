use tauri::Manager;

mod config;
mod database;
mod logging;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            database::bot_accounts::get_all_accounts,
            database::bot_accounts::insert_account,
            database::bot_accounts::update_account_token,
            database::bot_accounts::delete_account
        ])
        .setup(|app| {
            let config_path = format!(
                "{}\\UmbrageBot",
                app.path().config_dir().unwrap().to_str().unwrap()
            );

            // May this function never panic for the sake of my sanity
            logging::init(&config_path).unwrap();

            config::initialize(&config_path);
            database::initialize();

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
