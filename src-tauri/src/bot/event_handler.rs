use serenity::all::*;
use serenity::async_trait;

use crate::bot;
use crate::bot::account_manager::activity::ActivityWrapper;
use crate::database;
use crate::event_manager;
use crate::event_manager::events::BotLoginSuccessEvent;
use crate::logging::log_info;

pub struct EventHandler;


// See events here: https://docs.rs/serenity/latest/serenity/client/trait.EventHandler.html
#[async_trait]
impl serenity::prelude::EventHandler for EventHandler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        let current_user = ctx.http.get_current_user().await.unwrap();

        database::random::create_indexes(&current_user.id.to_string());

        bot::account_manager::initialize(&ctx).await;

        event_manager::emit(BotLoginSuccessEvent);

        log_info!("{} is connected!", ready.user.name);
    }

    async fn user_update(&self, ctx: Context, _: Option<CurrentUser>, new: CurrentUser) {
        let current_user = ctx.http.get_current_user().await.unwrap();

        if new.id == current_user.id {
            bot::account_manager::edit_profile(|profile| {
                profile.username = new.name.clone();
                profile.avatar_url = new.avatar_url().unwrap_or("".into());
                profile.banner_url = new.banner_url().unwrap_or("".into());
            });

            database::bot_accounts::update_account_info(&new);
        };
    }

    async fn presence_update(&self, ctx: Context, new_data: Presence) {
        let current_user = ctx.http.get_current_user().await.unwrap();

        if new_data.user.id == current_user.id {
            bot::account_manager::edit_profile(|profile| {
                profile.status = new_data.status;
                profile.activity = ActivityWrapper::from_presence(new_data);
            });
        };
    }
}