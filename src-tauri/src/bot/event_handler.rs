use serenity::all::*;
use serenity::async_trait;

use crate::bot::active_bot;
use crate::bot::BotStateExt;
use crate::config::app_config;
use crate::database::database;
use crate::logging::log_info;

pub struct EventHandler;

#[async_trait]
impl serenity::prelude::EventHandler for EventHandler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        let bot = active_bot();

        let current_user = ctx.http.get_current_user().await.unwrap();

        // account_manager::emit_user_update(&current_user);

        bot.lock_and_get().await.login_notify.notify_waiters();

        log_info!("{} is connected!", ready.user.name);
    }

    async fn user_update(&self, ctx: Context, _: Option<CurrentUser>, new: CurrentUser) {
        let current_user = ctx.http.get_current_user().await.unwrap();

        if new.id == current_user.id {
            //account_manager::emit_user_update(&new);
        };
    }
}