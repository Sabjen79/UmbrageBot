use std::{error::Error, sync::{Arc, OnceLock}};

use event_handler::EventHandler;
use serenity::{all::{GatewayIntents, Http, OnlineStatus, ShardMessenger}, Client};
use tauri::{Manager, State};
use tokio::sync::{MappedMutexGuard, Mutex, MutexGuard};

use crate::{app_config::{self}, app_handle, bot::account_manager::{activity::ActivityWrapper, BotProfile}, database::{self}, event_manager::{self, events::{BotShutdownStartEvent, BotShutdownSuccessEvent}}, logging::{log_error, log_info}, timer_manager};

mod event_handler;
pub mod account_manager;
pub mod commands;

pub struct Bot {
    http: Arc<Http>,
    shard_messenger: OnceLock<ShardMessenger>,
    profile: Mutex<BotProfile>
}

pub type BotState = Mutex<Option<Bot>>;

impl Bot {
    pub async fn start_bot(token: &str) -> Result<Bot, Box<dyn Error>> {
        let mut client =
            Client::builder(token, GatewayIntents::all())
                .event_handler(EventHandler)
                .status(OnlineStatus::Invisible)
                .await?;
        
        let user_res = client.http.get_current_user().await;

        let profile: BotProfile;
        
        match user_res {
            Ok(user) => {
                app_config::bot::initialize_bot_config(user.id.to_string()).await;
                
                database::bot_accounts::update_account_info(&user).await;

                profile = BotProfile {
                    username: user.name.clone(),
                    avatar_url: user.avatar_url().unwrap_or("".into()),
                    banner_url: user.banner_url().unwrap_or("".into()),
                    status: OnlineStatus::Invisible,
                    activity: ActivityWrapper::None
                }
            }
            Err(_) => {
                return Err("Invalid Bot Token".into());
            }
        }

        let _self = Self {
            http: client.http.clone(),
            shard_messenger: OnceLock::new(),
            profile: Mutex::new(profile)
        };

        // Shutdown Thread
        let manager = client.shard_manager.clone();

        let shutdown_handle = tokio::spawn(async move {
            event_manager::wait(BotShutdownStartEvent).await;
            
            manager.shutdown_all().await;

            event_manager::emit(BotShutdownSuccessEvent);

            log_info!("Bot shutdown successfully");
        });

        // Bot Thread
        tokio::spawn(async move {
            // This function runs continously until the bot is closed
            if let Err(why) = client.start().await { 
                log_error!("{}", why.to_string())
            }

            shutdown_handle.abort();

            event_manager::unlisten_all().await;
            timer_manager::cancel_all().await;
        });

        Ok(_self)
    }

    pub fn get_state() -> State<'static, BotState> {
        return app_handle().state::<BotState>();
    }

    pub fn shard_messenger(&self) -> &ShardMessenger {
        match self.shard_messenger.get() {
            Some(shard) => shard,
            None => panic!("shard_messenger is not initialized")
        }
    }

    pub fn shutdown() {
        event_manager::emit(BotShutdownStartEvent);
    }
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
            panic!("There is no bot active");
        }

        MutexGuard::map(guard, |opt| opt.as_mut().unwrap())
    }
}