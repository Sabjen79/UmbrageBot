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

pub fn edit_profile<F>(callback: F)
where
    F: FnOnce(&mut BotProfile),
{
    let state = Bot::get_state();
    let lock = state.lock().unwrap();
    let bot = lock.get_bot();

    let mut profile_lock = bot.profile.lock().unwrap();

    callback(&mut *profile_lock);

    event_manager::emit(BotProfileUpdateEvent {
        data: profile_lock.clone()
    });
}

pub async fn initialize(ctx: &Context) {
    let _current_user = ctx.http.get_current_user().await.unwrap();

    {
        let state = Bot::get_state();
        let lock = state.lock().unwrap();
        let bot = lock.get_bot();

        _ = bot.shard_messenger.set(Arc::new(ctx.shard.clone()));

        let profile = bot.profile.lock().unwrap();

        event_manager::emit(BotProfileUpdateEvent {
            data: profile.clone()
        });
    }

    let bot_config = app_config::bot::bot_config();

    _ = bot::account_manager::set_status(bot_config.bot_status).await;

    // Activity
    timer_manager::new_timer("BOT_ACTIVITY_TIMER")
        .duration_handler(|| async move {
            let bot_config = app_config::bot::bot_config();

            let x = rand::random_range(bot_config.activity_timer_min..=bot_config.activity_timer_max);

            Ok(Duration::from_secs(x.into()))
        })
        .action(|| async move {
            let bot_config = app_config::bot::bot_config();

            if !bot_config.activity_timer_enabled {
                return Ok(());
            }

            let activity = database::activities::get_random();

            set_activity(activity).await;

            Ok(())
        })
        .build_and_register()
        .start_and_run();

    event_manager::listen(async |event: BotConfigUpdateEvent| {
        let config = &event.new_config;
        if config.activity_timer_enabled != event.old_config.activity_timer_enabled {
            if config.activity_timer_enabled {
                if let Some(timer) = timer_manager::get_timer("BOT_ACTIVITY_TIMER") {
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

    let http = bot::http();

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

    let shard = bot::shard_messenger();

    shard.set_status(status);

    edit_profile(|profile| {
        profile.status = status;
    });

    app_config::bot::edit_bot_config(|config| {
        config.bot_status = status
    })?;

    Ok(())
}

pub async fn set_activity(activity: ActivityWrapper) {
    let shard = bot::shard_messenger();

    match activity.clone().into_data() {
        Ok(act) => {
            shard.set_activity(act.clone());

            edit_profile(|profile| {
                profile.activity = activity;
            });
        }
        Err(err) => {
            log_error!("Error setting activity: {}", err.to_string());
        }
    };
}