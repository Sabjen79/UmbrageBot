use std::{
    error::Error, fs::{self, OpenOptions}, io::Write, sync::OnceLock, time::{SystemTime, UNIX_EPOCH}
};

static LOG_FILE: OnceLock<String> = OnceLock::new();

macro_rules! log_info {
    ($($args: tt)*) => {
        let _ = crate::logging::write_log(format!($($args)*), "INFO");
        println!($($args)*);
    }
}

pub fn emit_error(error: String) {
    APP_HANDLE.get().unwrap().emit("error", error).unwrap();
}

macro_rules! log_error {
    ($($args: tt)*) => {
        let _ = crate::logging::write_log(format!($($args)*), "ERROR");
        println!($($args)*);

        crate::logging::emit_error(format!($($args)*));
    }
}

pub(crate) use log_info;
pub(crate) use log_error;
use tauri::Emitter;

use crate::APP_HANDLE;

pub fn init(config_path: &str) -> Result<(), Box<dyn Error>> {
    let dir_path = format!("{}{}", config_path, "\\logs");

    if !fs::exists(&config_path)? {
        fs::create_dir_all(config_path).unwrap();
    }

    let current_time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();

    if !fs::exists(&dir_path)? {
        fs::create_dir(&dir_path)?;
    }

    for file in fs::read_dir(&dir_path)? {
        let file = file?;
        let st: String = file.file_name().to_str().unwrap().chars().skip(4).collect();

        let date: u128 = match st.parse() {
            Ok(d) => d,
            Err(_) => {
                continue;
            }
        };

        let difference: u128 = 604_800_000; // 7 days
        if current_time - date >= difference {
            fs::remove_file(file.path())?;
        }
    }

    LOG_FILE.set(format!("{}\\log_{}.txt", &dir_path, current_time))?;

    log_info!("{}", "Logging Initialized. Hello!");

    Ok(())
}

pub fn write_log(message: String, level: &str) -> Result<(), Box<dyn Error>> {
    let path = LOG_FILE.get().unwrap();

    if !fs::exists(path)? {
        fs::File::create(path)?;
    }

    let log = format!(
        "[{}][{}] {}\n",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.6f"),
        level,
        message
    );

    let _ = OpenOptions::new()
        .append(true)
        .open(path)?
        .write(log.as_bytes());

    Ok(())
}

