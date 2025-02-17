use std::sync::{Arc, OnceLock};

use event_handler::EventHandler;
use serenity::{all::{GatewayIntents, Http}, Client};
use tokio::sync::Notify;

use crate::{database::bot_accounts::BotAccount, logging::error};

mod event_handler;
mod account_manager;

pub static ACTIVE_BOT: OnceLock<BotAccount> = OnceLock::new();
static SHUTDOWN_NOTIFY: OnceLock<Notify> = OnceLock::new();
static HTTP: OnceLock<Arc<Http>> = OnceLock::new();

// pub async fn get_user() -> CurrentUser {
//     let http = HTTP.get().unwrap();

//     http.get_current_user().await.unwrap()
// }

#[tauri::command]
pub async fn start_bot(token: &str) -> Result<(), String> {
    let account_res = crate::database::bot_accounts::get_bot_info(token).await;
    
    match account_res {
        Ok(account) => {
            ACTIVE_BOT.set(account).unwrap();

            tokio::spawn(async {
                start().await;
            });
            
            Ok(())
        },
        Err(err) => {
            Err(err.to_string())
        }
    }
}

async fn start() {
    crate::config::initialize_bot_config().await;

    let mut client =
        Client::builder(&ACTIVE_BOT.get().unwrap().token, GatewayIntents::all())
            .event_handler(EventHandler)
            .await
            .expect("Err creating client");

    HTTP.set(client.http.clone()).unwrap();

    // account_manager::initialize();

    SHUTDOWN_NOTIFY.set(Notify::new()).unwrap();

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        SHUTDOWN_NOTIFY.get().unwrap().notified().await;
        
        shard_manager.shutdown_all().await;

        std::process::exit(0);
    });

    if let Err(why) = client.start().await {
        error!("Discord client error: {why:?}");
    }
}

pub async fn shutdown() {
    match SHUTDOWN_NOTIFY.get() {
        Some(notify) => {
            notify.notify_one();
        },
        None => {
            std::process::exit(0);
        }
    }
}