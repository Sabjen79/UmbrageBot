use std::{error::Error, sync::{Arc, LazyLock, Mutex}};

use event_handler::EventHandler;
use serenity::{all::GatewayIntents, Client};
use tokio::sync::Notify;

use crate::logging::{log_error, log_info};

mod event_handler;
mod account_manager;

pub static LOGIN_NOTIFY: LazyLock<Notify> = LazyLock::new(|| Notify::new());
pub static LOGOUT_NOTIFY: LazyLock<Notify> = LazyLock::new(|| Notify::new());

#[tauri::command]
pub async fn start_bot(token: String) -> Result<(), ()> {
    let is_ok = Arc::new(Mutex::new(true));
    let is_ok_clone = Arc::clone(&is_ok);

    tokio::spawn(async move {
        match start(token.as_str()).await {
            Ok(_) => {
                // Idc about this case since it happens only on manual shutdown
            },
            Err(err) => {
                log_error!("{}", err.to_string());

                *is_ok_clone.lock().unwrap() = false;

                LOGIN_NOTIFY.notify_waiters();
            }
        }
    });

    LOGIN_NOTIFY.notified().await;

    if *is_ok.lock().unwrap() { 
        Ok(())
    } else { 
        Err(())
    }
}

#[tauri::command]
pub async fn logout() {
    LOGOUT_NOTIFY.notify_waiters();
}


async fn start(token: &str) -> Result<(), Box<dyn Error>> {
    let mut client =
        Client::builder(token, GatewayIntents::all())
            .event_handler(EventHandler)
            .await?;

    let user = client.http.get_current_user().await
        .map_err(|e| format!("Invalid bot token ({})", e))?;

    crate::config::initialize_bot_config(&user.id.to_string()).await;

    // account_manager::initialize();

    let shard_manager = client.shard_manager.clone();

    let logout_handle = tokio::spawn(async move {
        LOGOUT_NOTIFY.notified().await;
        
        shard_manager.shutdown_all().await;

        log_info!("Bot logged out successfully");
    });

    if let Err(why) = client.start().await {
        return Err(Box::new(why));
    }

    logout_handle.abort();

    Ok(())
}