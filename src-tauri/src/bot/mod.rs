use std::{error::Error, sync::{Arc, LazyLock, Mutex, RwLock}};

use event_handler::EventHandler;
use serenity::{all::{GatewayIntents, Http}, Client};
use tauri::Listener;
use tokio::sync::Notify;

use crate::{logging::{log_error, log_info}, APP_HANDLE};

mod event_handler;
pub mod account_manager;

pub static LOGIN_NOTIFY: LazyLock<Notify> = LazyLock::new(|| Notify::new());
pub static LOGOUT_NOTIFY: LazyLock<Notify> = LazyLock::new(|| Notify::new());
pub static LISTENERS: Mutex<Vec<u32>> = Mutex::new(Vec::new());

static HTTP: RwLock<Option<Arc<Http>>> = RwLock::new(None);

#[tauri::command]
pub async fn start_bot(token: String) -> Result<(), ()> {
    let is_ok = Arc::new(Mutex::new(true));
    let is_ok_clone = Arc::clone(&is_ok);

    tokio::spawn(async move {
        match start(token.as_str()).await {
            Ok(_) => {
                // Idc about this case since it happens only on manual shutdown
            },
            Err(err) => {
                log_error!("{}", err.to_string());

                *is_ok_clone.lock().unwrap() = false;

                LOGIN_NOTIFY.notify_waiters();
            }
        }
    });

    LOGIN_NOTIFY.notified().await;

    LISTENERS.lock().unwrap().clear();

    if *is_ok.lock().unwrap() { 
        Ok(())
    } else { 
        Err(())
    }
}

#[tauri::command]
pub async fn logout() {
    LOGOUT_NOTIFY.notify_waiters();
}

async fn start(token: &str) -> Result<(), Box<dyn Error>> {
    let mut client =
        Client::builder(token, GatewayIntents::all())
            .event_handler(EventHandler)
            .await?;

    *HTTP.write().unwrap() = Some(client.http.clone());

    let user = client.http.get_current_user().await
        .map_err(|e| format!("Invalid bot token ({})", e))?;


    let shard_manager = client.shard_manager.clone();

    let logout_handle = tokio::spawn(async move {
        LOGOUT_NOTIFY.notified().await;
        
        shard_manager.shutdown_all().await;

        log_info!("Bot logged out successfully");
    });

    crate::config::initialize_bot_config(&user.id.to_string()).await;

    account_manager::initialize().await;

    if let Err(why) = client.start().await {
        return Err(Box::new(why));
    }

    logout_handle.abort();

    for event in &*LISTENERS.lock().unwrap() {
        APP_HANDLE.get().unwrap().unlisten(*event);
    }

    *HTTP.write().unwrap() = None;

    Ok(())
}

/// This function should be used only by listeners that are canceled when the
/// client object no longer exists
fn get_http() -> Arc<Http> {
    match &*HTTP.read().unwrap() {
        Some(arc_http) => {
            arc_http.clone()
        }
        None => {
            panic!("Client is not initialized!");
        }
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