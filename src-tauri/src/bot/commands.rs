use crate::{bot::{self, Bot}, event_manager::{self, events::{BotLoginSuccessEvent, BotShutdownSuccessEvent}}, logging::log_error};

#[tauri::command]
pub async fn start_bot(token: String) -> Result<(), String> {
    match Bot::start_bot(token.as_str()).await.map_err(|e| e.to_string()) {
        Ok(bot) => {
            {
                let state = Bot::get_state();
                let mut lock = state.lock().await;
                *lock = Some(bot);
            }

            event_manager::wait(BotLoginSuccessEvent).await;

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
    let bot_state = Bot::get_state();
    
    {
        if let Some(_) = &*bot_state.lock().await {
            bot::shutdown();

            event_manager::wait(BotShutdownSuccessEvent).await;
        }

        *bot_state.lock().await = None;
    }

    Ok(())
}