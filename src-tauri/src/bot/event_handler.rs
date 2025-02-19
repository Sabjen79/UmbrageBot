use serenity::all::*;
use serenity::async_trait;

use crate::bot::LOGIN_NOTIFY;
use crate::logging::log_info;

pub struct EventHandler;

#[async_trait]
impl serenity::prelude::EventHandler for EventHandler {
    async fn ready(&self, _: Context, ready: Ready) {
        LOGIN_NOTIFY.notify_waiters();

        log_info!("{} is connected!", ready.user.name);
    }
}