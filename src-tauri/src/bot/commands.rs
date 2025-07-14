use crate::{bot::{active_bot, Bot}, logging::log_error};

#[tauri::command]
pub async fn start_bot(token: String) -> Result<(), String> {
    match Bot::start_bot(token.as_str()).await.map_err(|e| e.to_string()) {
        Ok(bot) => {
            let notify = bot.login_notify.clone();

            let state = active_bot();
            let mut lock = state.lock().await;
            *lock = Some(bot);
            drop(lock);
            
            notify.notified().await;

            return Ok(());
        }

        Err(err) => {
            log_error!("{}", err);

            return Err(err);
        }
    }
}

#[tauri::command]
pub async fn shutdown_bot() -> Result<(), String> {
    let bot_state = active_bot();
    
    if let Some(bot) = &*bot_state.lock().await {
        bot.shutdown();
    }

    *bot_state.lock().await = None;

    Ok(())
}