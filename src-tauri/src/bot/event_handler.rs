use serenity::all::*;
use serenity::async_trait;

use crate::logging::log_info;

pub struct EventHandler;

#[async_trait]
impl serenity::prelude::EventHandler for EventHandler {
    async fn ready(&self, _: Context, ready: Ready) {
        log_info!("{} is connected!", ready.user.name);
    }
}