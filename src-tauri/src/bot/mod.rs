use std::sync::OnceLock;

use event_handler::EventHandler;
use serenity::{all::GatewayIntents, Client};
use tokio::sync::Notify;

use crate::{database::bot_accounts::BotAccount, logging::error};

static ACTIVE_BOT: OnceLock<BotAccount> = OnceLock::new();
static SHUTDOWN_NOTIFY: OnceLock<Notify> = OnceLock::new();

mod event_handler;

#[tauri::command]
pub async fn start_bot(token: &str) -> Result<(), String> {
    let account_res = crate::database::bot_accounts::get_bot_info(token).await;
    
    match account_res {
        Ok(account) => {
            ACTIVE_BOT.get_or_init(|| account);

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
    let mut client =
        Client::builder(&ACTIVE_BOT.get().unwrap().token, GatewayIntents::all())
            .event_handler(EventHandler)
            .await
            .expect("Err creating client");

    SHUTDOWN_NOTIFY.get_or_init(|| Notify::new());

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
        None => {}
    }
}