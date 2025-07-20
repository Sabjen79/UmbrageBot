use std::fmt::Debug;

use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};

use crate::bot::account_manager::BotProfile;

/// Trait that adds the function `name()` to a struct to get its type name in UPPER_SNAKE_CASE
pub trait TauriEvent {
    fn name() -> String;
}

macro_rules! impl_type_name {
    ($($ty:ty),*) => {
        $(impl TauriEvent for $ty {
            fn name() -> String {
                let s = stringify!($ty).to_string();

                s.to_case(Case::Constant)
            }
        })*
    }
}

// REMEMBER: Add all struct events in this function to implement the event trait
impl_type_name!(
    BotLoginSuccessEvent,
    BotShutdownStartEvent,
    BotShutdownSuccessEvent,
    BotProfileUpdateEvent
);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotLoginSuccessEvent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotShutdownStartEvent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotShutdownSuccessEvent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotProfileUpdateEvent {
    pub data: BotProfile
}

