use std::{collections::HashMap, sync::Arc};

use tauri::{Manager, State};
use tokio::sync::Mutex;

use crate::{app_handle, timer_manager::timer::{Timer, TimerBuilder}};

pub mod timer;
pub mod commands;

pub struct TimerManager {
    timers: Mutex<HashMap<String, Arc<Timer>>>
}

impl TimerManager {
    pub fn new() -> TimerManager {
        Self { 
            timers: Mutex::new(HashMap::new())
        }
    }

    pub(in crate::timer_manager)
    fn get_state() -> State<'static, TimerManager> {
        app_handle().state::<TimerManager>()
    }
}

pub fn new_timer(name: &str) -> TimerBuilder
{
    TimerBuilder::new(name)
}

pub async fn get_timer(name: &str) -> Option<Arc<Timer>> {
    let state = TimerManager::get_state();

    let timers = state.timers.lock().await;

    timers.get(name).map(|t| t.clone())
}

pub async fn cancel_all() {
    let state = TimerManager::get_state();

    let mut timers = state.timers.lock().await;

    timers.iter().for_each(|t| t.1.cancel());

    timers.clear();
}

pub(in crate::timer_manager)
async fn register_timer(name: &str, timer: Arc<Timer>) {
    let state = TimerManager::get_state();

    let mut timers = state.timers.lock().await;
    timers.insert(name.to_string(), timer);
}