use std::error::Error;

use serde::{Deserialize, Serialize};
use serenity::all::{EditProfile, OnlineStatus};

use crate::{bot::{account_manager::activity::ActivityWrapper, Bot, BotStateExt}, app_config, event_manager::{self, events::BotProfileUpdateEvent}};

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