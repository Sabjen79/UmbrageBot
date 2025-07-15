use tauri::{Emitter, Listener};
use tokio::sync::oneshot::{self};

use crate::{app_handle, logging::log_error};

pub mod events;

pub struct EventManager {
    
}

// pub fn event_manager() -> State<'static, EventManager> {
//     app_handle().state::<EventManager>()
// }

/// Wrapper around tauri events, use this instead of calling `AppHandle` functions
impl EventManager {
    pub fn new() -> EventManager {
        Self { 
            
        }
    }

    pub fn emit_notify(event: events::NotifyEvents) {
        app_handle()
            .emit(event.to_string().as_str(), ())
            .unwrap_or_else(|err| {
                log_error!("Cannot emit event: {}", err.to_string());
            });
    }

    pub async fn wait_notify(event: events::NotifyEvents) {
        let (sx, rx) = oneshot::channel();

        app_handle().once(event.to_string().as_str(), |_| {
            sx.send(()).unwrap();
        });

        rx.await.unwrap();
    }
}