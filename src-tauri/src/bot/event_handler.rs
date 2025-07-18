use serenity::all::*;
use serenity::async_trait;

use crate::app_config::AppConfiguration;
use crate::bot;
use crate::bot::account_manager::activity::ActivityWrapper;
use crate::bot::Bot;
use crate::bot::BotStateExt;
use crate::database;
use crate::event_manager;
use crate::event_manager::events::BotLoginSuccessEvent;
use crate::event_manager::events::BotProfileUpdateEvent;
use crate::logging::log_info;

pub struct EventHandler;


// See events here: https://docs.rs/serenity/latest/serenity/client/trait.EventHandler.html
#[async_trait]
impl serenity::prelude::EventHandler for EventHandler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        let state = Bot::get_state();

        let _current_user = ctx.http.get_current_user().await.unwrap();

        event_manager::emit(BotLoginSuccessEvent);

        {
            let bot = state.lock_and_get().await;

            _ = bot.shard_messenger.set(ctx.shard);

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

        log_info!("{} is connected!", ready.user.name);
    }

    async fn user_update(&self, ctx: Context, _: Option<CurrentUser>, new: CurrentUser) {
        let current_user = ctx.http.get_current_user().await.unwrap();

        if new.id == current_user.id {
            let state = Bot::get_state();
            let bot = state.lock_and_get().await;
            let mut profile = bot.profile.lock().await;

            profile.username = new.name.clone();
            profile.avatar_url = new.avatar_url().unwrap_or("".into());
            profile.banner_url = new.banner_url().unwrap_or("".into());

            event_manager::emit(BotProfileUpdateEvent {
                data: profile.clone()
            });

            database::bot_accounts::update_account_info(&new).await;
        };
    }

    async fn presence_update(&self, ctx: Context, new_data: Presence) {
        let current_user = ctx.http.get_current_user().await.unwrap();

        if new_data.user.id == current_user.id {
            let state = Bot::get_state();
            let bot = state.lock_and_get().await;
            let mut profile = bot.profile.lock().await;

            profile.status = new_data.status;
            profile.activity = ActivityWrapper::from_data(new_data);

            event_manager::emit(BotProfileUpdateEvent {
                data: profile.clone()
            });
        };
    }
}