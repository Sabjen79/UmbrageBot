use std::{error::Error, sync::Arc, time::Duration};

use serde::{Deserialize, Serialize};
use serenity::all::{Context, EditProfile, OnlineStatus};

use crate::{app_config::{self}, bot::{self, account_manager::activity::ActivityWrapper, Bot, BotStateExt}, database, event_manager::{self, events::{BotConfigUpdateEvent, BotProfileUpdateEvent}}, logging::log_error, timer_manager};

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

pub async fn edit_profile<F>(callback: F)
where
    F: FnOnce(&mut BotProfile),
{
    let state = Bot::get_state();

    {
        let bot = state.lock_and_get().await;
        let mut lock = bot.profile.lock().await;

        callback(&mut *lock);

        event_manager::emit(BotProfileUpdateEvent {
            data: lock.clone()
        });
    }
}

pub async fn initialize(ctx: &Context) {
    let state = Bot::get_state();
    let _current_user = ctx.http.get_current_user().await.unwrap();

    {
        let bot = state.lock_and_get().await;

        _ = bot.shard_messenger.set(Arc::new(ctx.shard.clone()));

        let profile = bot.profile.lock().await;

        event_manager::emit(BotProfileUpdateEvent {
            data: profile.clone()
        });
    }

    let bot_config = app_config::bot::bot_config().await;

    _ = bot::account_manager::set_status(bot_config.bot_status).await;

    // Activity
    timer_manager::new_timer("BOT_ACTIVITY_TIMER")
        .duration_handler(|| async move {
            let bot_config = app_config::bot::bot_config().await;

            let x = rand::random_range(bot_config.activity_timer_min..=bot_config.activity_timer_max);

            Ok(Duration::from_secs(x.into()))
        })
        .action(|| async move {
            let bot_config = app_config::bot::bot_config().await;

            if !bot_config.activity_timer_enabled {
                return Ok(());
            }

            let activity = database::activities::get_random().await;

            set_activity(activity).await;

            Ok(())
        })
        .build_and_register()
        .start_and_run();

    event_manager::listen(async |event: BotConfigUpdateEvent| {
        let config = &event.new_config;
        if config.activity_timer_enabled != event.old_config.activity_timer_enabled {
            if config.activity_timer_enabled {
                if let Some(timer) = timer_manager::get_timer("BOT_ACTIVITY_TIMER").await {
                    timer.run_early();
                }
            } else {
                set_activity(ActivityWrapper::None).await;
            }
        }
    });
}

pub async fn set_username(username: &str) -> Result<(), Box<dyn Error>> {
    if username.is_empty() {
        return Err("Username cannot be empty".into());
    }

    let http = bot::http().await;

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

    let shard = bot::shard_messenger().await;

    shard.set_status(status);

    edit_profile(|profile| {
        profile.status = status;
    }).await;

    app_config::bot::edit_bot_config(|config| {
        config.bot_status = status
    }).await?;

    Ok(())
}

pub async fn set_activity(activity: ActivityWrapper) {
    let shard = bot::shard_messenger().await;

    match activity.clone().into_data() {
        Ok(act) => {
            shard.set_activity(act.clone());

            edit_profile(|profile| {
                profile.activity = activity;
            }).await;
        }
        Err(err) => {
            log_error!("Error setting activity: {}", err.to_string());
        }
    };
}