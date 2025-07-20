use std::{error::Error, time::Duration};

use serde::{Deserialize, Serialize};
use serenity::all::{Context, EditProfile, OnlineStatus};

use crate::{app_config::{self, AppConfiguration}, bot::{self, account_manager::activity::ActivityWrapper, Bot, BotStateExt}, event_manager::{self, events::BotProfileUpdateEvent}, timer_manager};

pub mod activity;
pub mod commands;

// This struct should reflect the store in Svelte
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct BotProfile {
    pub username: String,
    pub avatar_url: String,
    pub banner_url: String,
    pub status: OnlineStatus,
    pub activity: ActivityWrapper
}

pub async fn initialize(ctx: &Context) {
    let state = Bot::get_state();
    let _current_user = ctx.http.get_current_user().await.unwrap();

    {
        let bot = state.lock_and_get().await;

        _ = bot.shard_messenger.set(ctx.shard.clone());

        let profile = bot.profile.lock().await;

        event_manager::emit(BotProfileUpdateEvent {
            data: profile.clone()
        });
    }

    let bot_config = {
        let app_config = AppConfiguration::get_state();

        app_config.bot_config.lock().await.clone()
    };

    _ = bot::account_manager::set_status(bot_config.bot_status).await;

    timer_manager::new_timer("BOT_TEST_TIMER")
        .action(|| async move {
            println!("DAB");
        })
        .duration_handler(|| async move {
            Duration::from_secs(3610)
        })
        .build_and_register()
        .start();
}

pub async fn set_username(username: &str) -> Result<(), Box<dyn Error>> {
    if username.is_empty() {
        return Err("Username cannot be empty".into());
    }

    let state = Bot::get_state();

    let http = {
        let bot = state.lock_and_get().await;

        bot.http.clone()
    };

    let mut user = http.get_current_user().await?;

    user.edit(&http, EditProfile::new().username(username)).await
        .map_err(|err| {
            let mut e = err.to_string();

            if e.contains("username: ") {
                e = e.split("username: ").last().unwrap().to_string();
                e.pop();

                return e;
            }

            e
        })?;

    Ok(())
}

pub async fn set_status(status: OnlineStatus) -> Result<(), Box<dyn Error>> {
    if status == OnlineStatus::Offline {
        return Err("Offline status is not valid".into());
    }

    {
        let state = Bot::get_state();
        let bot = state.lock_and_get().await;
        let shard = bot.shard_messenger();

        shard.set_status(status);

        // For some reason, the bot doesn't receive status events for itself in EventHandler
        // Therefore: manual changes
        let mut profile = bot.profile.lock().await;

        profile.status = status;

        // If a fix is found, remember to move this back to EventHandler
        event_manager::emit(BotProfileUpdateEvent {
            data: profile.clone()
        });
    }

    app_config::bot::edit_bot_config(|config| {
        config.bot_status = status
    }).await?;

    Ok(())
}