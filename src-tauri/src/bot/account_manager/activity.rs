use core::error;

use serde::{Deserialize, Serialize};
use serenity::all::{ActivityData, ActivityType, Presence};
use tauri::Url;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum ActivityWrapper {
    None,
    Playing(String),
    Streaming(String, String),
    Listening(String),
    Watching(String),
    Custom(String),
    Competing(String)
}

impl ActivityWrapper {
    pub fn from_data(presence: Presence) -> ActivityWrapper {
        if presence.activities.is_empty() {
            return ActivityWrapper::None;
        }

        let activity = presence.activities.first().unwrap();

        match activity.kind {
            ActivityType::Playing => ActivityWrapper::Playing(activity.name.clone()),
            ActivityType::Listening => ActivityWrapper::Listening(activity.name.clone()),
            ActivityType::Competing => ActivityWrapper::Competing(activity.name.clone()),
            ActivityType::Watching => ActivityWrapper::Watching(activity.name.clone()),
            ActivityType::Streaming => ActivityWrapper::Streaming(
                activity.name.clone(), 
                activity.url.clone().unwrap_or(Url::parse("").unwrap()).to_string()
            ),
            ActivityType::Custom => ActivityWrapper::Custom(
                activity.state.clone().unwrap_or("".into()).to_string()
            ),
            _ => ActivityWrapper::None
        }
    }

    pub fn into_data(self) -> Result<Option<ActivityData>, Box<dyn error::Error>> {
        match self {
            ActivityWrapper::Playing(name) => Ok(Some(ActivityData {
                name,
                kind: ActivityType::Playing,
                state: None,
                url: None,
            })),
            ActivityWrapper::Streaming(name, url) => {
                let parsed_url = Url::parse(&url)?;
                Ok(Some(ActivityData {
                    name,
                    kind: ActivityType::Streaming,
                    state: None,
                    url: Some(parsed_url),
                }))
            }
            ActivityWrapper::Listening(name) => Ok(Some(ActivityData {
                name,
                kind: ActivityType::Listening,
                state: None,
                url: None,
            })),
            ActivityWrapper::Watching(name) => Ok(Some(ActivityData {
                name,
                kind: ActivityType::Watching,
                state: None,
                url: None,
            })),
            ActivityWrapper::Custom(name) => Ok(Some(ActivityData {
                name: name.clone(),
                kind: ActivityType::Custom,
                state: Some(name),
                url: None,
            })),
            ActivityWrapper::Competing(name) => Ok(Some(ActivityData {
                name,
                kind: ActivityType::Competing,
                state: None,
                url: None,
            })),
            ActivityWrapper::None => Ok(None)
        }
    }
}