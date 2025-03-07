use std::sync::OnceLock;

use tauri::{menu::{Menu, MenuItem}, tray::{MouseButton, TrayIconBuilder, TrayIconEvent}, AppHandle, Manager};
use tauri_plugin_window_state::{AppHandleExt, StateFlags};

mod config;
mod database;
mod logging;
mod bot;

static APP_HANDLE: OnceLock<AppHandle> = OnceLock::new();

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            database::bot_accounts::validate_token,
            database::bot_accounts::get_all_accounts,
            database::bot_accounts::insert_account,
            database::bot_accounts::update_account_token,
            database::bot_accounts::delete_account,

            config::bot_config::get_bot_config,
            config::bot_config::set_bot_config,
            
            bot::start_bot,
            bot::logout,
            bot::account_manager::change_username
        ])
        .setup(|app| {
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit_i])?;

            TrayIconBuilder::new()
                .tooltip("Umbrage Bot")
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        app.save_window_state(StateFlags::all()).unwrap_or(());

                        tokio::spawn(async move {
                            bot::logout().await;
                            exit_app().await;
                        });
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| match event {
                    TrayIconEvent::DoubleClick {
                      button: MouseButton::Left,
                      ..
                    } => {
                      let app = tray.app_handle();
                      if let Some(window) = app.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                      }
                    }
                    _ => {}
                })
                .build(app)?;

            APP_HANDLE.set(app.handle().to_owned()).unwrap();
            
            let config_path = format!(
                "{}\\Umbrage Bot",
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

pub fn get_app_handle() -> &'static AppHandle {
    return APP_HANDLE.get().unwrap();
}

pub async fn exit_app() {
    std::process::exit(0);
}