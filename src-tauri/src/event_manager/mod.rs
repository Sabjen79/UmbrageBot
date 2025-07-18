use std::fmt::Debug;

use serde::{de::DeserializeOwned, Serialize};
use tauri::{Emitter, Listener, Manager, State};
use tokio::{runtime::Handle, sync::{oneshot::{self}, Mutex}};

use crate::{app_handle, event_manager::events::TauriEvent, logging::log_error};

pub mod events;

pub struct EventManager {
    event_listeners: Mutex<Vec<u32>>,
}

/// Wrapper around tauri events, use this instead of calling `AppHandle` functions
impl EventManager {
    pub fn new() -> EventManager {
        Self { 
            event_listeners: Mutex::new(Vec::new())
        }
    }

    pub fn get_state() -> State<'static, EventManager> {
        app_handle().state::<EventManager>()
    }
}

pub fn emit<T: TauriEvent + Serialize + Clone>(event: T) {
    app_handle()
        .emit(T::name().as_str(), event)
        .unwrap_or_else(|err| {
            log_error!("Cannot emit event: {}", err.to_string());
        });
}

pub async fn wait<T: TauriEvent + Debug + Send + 'static>(event: T) -> T {
    let (sx, rx) = oneshot::channel();

    app_handle().once(T::name().as_str(), move |_| {
        sx.send(event).unwrap();
    });

    rx.await.unwrap()
}

pub fn listen<T: TauriEvent + DeserializeOwned, F, Fut>(handler: F)
where 
    F: Fn(T) -> Fut + Send + 'static + Copy,
    Fut: Future<Output = ()> + Send + 'static,
{
    let event_id = app_handle().listen(T::name(), move |ev| {
        let payload = ev.payload();
        match serde_json::from_str::<T>(payload) {
            Ok(value) => {
                tokio::task::block_in_place(move || {
                    Handle::current().block_on(async move {
                        handler(value).await;
                    });
                });
            },
            Err(err) => {
                log_error!("Failed to parse event payload: {}", err);
            }
        }
    });
    
    // Store the event_id for later cleanup
    tokio::spawn(async move {
        let em = EventManager::get_state();
        em.event_listeners.lock().await.push(event_id);
    });
}

pub async fn unlisten_all() {
    let em = EventManager::get_state();
    let mut vec = em.event_listeners.lock().await;

    for event in &*vec {
        app_handle().unlisten(*event);
    }

    (*vec).clear();
}