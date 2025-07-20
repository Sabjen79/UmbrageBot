use crate::timer_manager;

#[tauri::command]
pub async fn timer_run_early(name: String) -> Result<(), String> {
    match timer_manager::get_timer(name.as_str()).await {
        Some(timer) => {
            timer.run_early();

            Ok(())
        }
        None => Err(format!("Invalid timer name: {}", name))
    }
}

#[tauri::command]
pub async fn timer_reset(name: String) -> Result<(), String> {
    match timer_manager::get_timer(name.as_str()).await {
        Some(timer) => {
            timer.reset();

            Ok(())
        }
        None => Err(format!("Invalid timer name: {}", name))
    }
}

#[tauri::command]
pub async fn timer_get_time_left(name: String) -> Result<u128, String> {
    match timer_manager::get_timer(name.as_str()).await {
        Some(timer) => {
            Ok(timer.get_time_left().await)
        }
        None => Err(format!("Invalid timer name: {}", name))
    }
}