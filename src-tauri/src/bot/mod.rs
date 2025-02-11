use std::sync::OnceLock;

use serenity::{all::GatewayIntents, Client};

use crate::database::bot_accounts::BotAccount;


static ACTIVE_BOT: OnceLock<BotAccount> = OnceLock::new();

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
        Client::builder(&ACTIVE_BOT.get().unwrap().token, GatewayIntents::all()).await.expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}