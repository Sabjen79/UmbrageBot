use std::{error::Error, sync::Arc};

use event_handler::EventHandler;
use serenity::{all::{GatewayIntents, Http}, Client};
use tauri::{Listener, Manager, State};
use tokio::sync::{MappedMutexGuard, Mutex, MutexGuard, Notify};

use crate::{app_handle, config::app_config, database::database, logging::{log_error, log_info}};

mod event_handler;
pub mod account_manager;
pub mod commands;

pub struct Bot {
    login_notify: Arc<Notify>,
    shutdown_notify: Arc<Notify>,
    tauri_event_listeners: Arc<Mutex<Vec<u32>>>,
    http: Arc<Http>
}

pub type BotState = Mutex<Option<Bot>>;

impl Bot {
    pub async fn start_bot(token: &str) -> Result<Bot, Box<dyn Error>> {
        let mut client =
            Client::builder(token, GatewayIntents::all())
                .event_handler(EventHandler)
                .await?;
        
        let user_res = client.http.get_current_user().await;

        match user_res {
            Ok(user) => {
                app_config().initialize_bot_config(user.id.to_string()).await;
                
                database().update_account_info(&user).await;
            }
            Err(_) => {
                return Err("Invalid Bot Token".into());
            }
        }

        let manager = client.shard_manager.clone();

        let _self = Self {
            login_notify: Arc::new(Notify::new()),
            shutdown_notify: Arc::new(Notify::new()), 
            tauri_event_listeners: Arc::new(Mutex::new(Vec::new())), 
            http: client.http.clone()
        };

        // Shutdown Thread
        let manager_clone = manager.clone();
        let shutdown_notify = _self.shutdown_notify.clone();

        let shutdown_handle = tokio::spawn(async move {
            // TODO: Convert notifiers to tauri events
            shutdown_notify.notified().await;
            
            manager_clone.shutdown_all().await;

            log_info!("Bot shutdown successfully");
        });

        // Bot Thread
        let listeners = _self.tauri_event_listeners.clone();

        tokio::spawn(async move {
            // This function runs continously until the bot is closed
            if let Err(why) = client.start().await { 
                log_error!("{}", why.to_string())
            }

            shutdown_handle.abort();

            for event in &*listeners.lock().await {
                app_handle().unlisten(*event);
            }
        });

        Ok(_self)
    }

    pub fn shutdown(&self) {
        self.shutdown_notify.notify_waiters();
    }
}


pub fn active_bot() -> State<'static, BotState> {
    return app_handle().state::<BotState>();
}

pub trait BotStateExt {
    async fn lock_and_get(&self) -> MappedMutexGuard<'_, Bot>;
}

impl BotStateExt for BotState {
    /// Converts the `MutexGuard<'_, Option<Bot>>` into `MappedMutexGuard<'_, Bot>`.
    /// Will panic if there is no active bot
    async fn lock_and_get(&self) -> MappedMutexGuard<'_, Bot> {
        let guard = self.lock().await;

        if guard.is_none() {
            panic!("There is no bot active")
        }

        MutexGuard::map(guard, |opt| opt.as_mut().unwrap())
    }
}

// Always use this function for tauri events to make sure they are unsubscribed upon logout
// pub fn listen<F, Fut>(event_name: &str, handler: F)
// where 
//     F: Fn(Event, Arc<Http>) -> Fut + Send + 'static + Clone,
//     Fut: Future<Output = ()> + Send + 'static,
// {
//     let event_id = APP_HANDLE.get().unwrap().listen(event_name, move |event| {
//         let handler = handler.clone();
//         tokio::task::block_in_place(move || {
//             Handle::current().block_on(async move {
//                 handler(event, get_http()).await;
//             });
//         });
//     });

//     LISTENERS.lock().unwrap().push(event_id);
// }