use crate::{bot::{active_bot, Bot}, event_manager::{events::{BotLoginSuccessEvent, BotShutdownSuccessEvent}, EventManager}, logging::log_error};

#[tauri::command]
pub async fn start_bot(token: String) -> Result<(), String> {
    match Bot::start_bot(token.as_str()).await.map_err(|e| e.to_string()) {
        Ok(bot) => {
            {
                let state = active_bot();
                let mut lock = state.lock().await;
                *lock = Some(bot);
            }

            EventManager::wait(BotLoginSuccessEvent).await;

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
    
    {
        if let Some(bot) = &*bot_state.lock().await {
            bot.shutdown();

            EventManager::wait(BotShutdownSuccessEvent).await;
        }

        *bot_state.lock().await = None;
    }

    Ok(())
}