use crate::{bot::{active_bot, Bot}, event_manager::{events::NotifyEvents, EventManager}, logging::log_error};

#[tauri::command]
pub async fn start_bot(token: String) -> Result<(), String> {
    match Bot::start_bot(token.as_str()).await.map_err(|e| e.to_string()) {
        Ok(bot) => {
            {
                let state = active_bot();
                let mut lock = state.lock().await;
                *lock = Some(bot);
            }

            EventManager::wait_notify(NotifyEvents::BotLoginSuccess).await;

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
        }

        *bot_state.lock().await = None;
    }
    
    EventManager::wait_notify(NotifyEvents::BotShutdownSuccess).await;

    Ok(())
}