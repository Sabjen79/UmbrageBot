use serenity::all::*;
use serenity::async_trait;

use crate::bot::LOGIN_NOTIFY;
use crate::logging::log_info;

use super::account_manager;

pub struct EventHandler;

#[async_trait]
impl serenity::prelude::EventHandler for EventHandler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        let current_user = ctx.http.get_current_user().await.unwrap();

        account_manager::emit_user_update(&current_user);

        LOGIN_NOTIFY.notify_waiters();

        log_info!("{} is connected!", ready.user.name);
    }

    async fn user_update(&self, ctx: Context, _: Option<CurrentUser>, new: CurrentUser) {
        let current_user = ctx.http.get_current_user().await.unwrap();

        if new.id == current_user.id {
            account_manager::emit_user_update(&new);
        };
    }
}