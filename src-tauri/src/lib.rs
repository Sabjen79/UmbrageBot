use std::sync::OnceLock;

use tauri::{menu::{Menu, MenuItem}, tray::{MouseButton, TrayIconBuilder, TrayIconEvent}, AppHandle, Manager};
use tauri_plugin_window_state::{AppHandleExt, StateFlags};
use tokio::sync::Mutex;

use crate::{bot::Bot, app_config::AppConfiguration, database::Database, event_manager::EventManager};

mod app_config;
mod database;
mod logging;
mod bot;
mod event_manager;

/// Use `app_handle()` instead!
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
            database::commands::db_validate_token,
            database::commands::db_get_all_accounts,
            database::commands::db_insert_account,
            database::commands::db_update_account,
            database::commands::db_delete_account,

            app_config::commands::get_bot_config,
            app_config::commands::set_bot_config,
            
            bot::commands::start_bot,
            bot::commands::shutdown_bot,
            bot::account_manager::commands::bot_set_username,
            bot::account_manager::commands::bot_set_status
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
                            _ = bot::commands::shutdown_bot().await;
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

            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                app_handle.manage(EventManager::new());
                app_handle.manage(AppConfiguration::new(&config_path));
                app_handle.manage(Database::new().await);
                app_handle.manage(Mutex::new(Option::<Bot>::None))
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub fn app_handle() -> &'static AppHandle {
    APP_HANDLE.get().unwrap()
}

pub async fn exit_app() {
    std::process::exit(0);
}