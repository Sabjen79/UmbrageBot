use serenity::all::*;
use serenity::async_trait;

use crate::logging::info;

pub struct EventHandler;

#[async_trait]
impl serenity::prelude::EventHandler for EventHandler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}