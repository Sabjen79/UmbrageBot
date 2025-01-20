use std::{fs, path::Path, sync::OnceLock};
use rusqlite::Connection;

pub mod bot_accounts;

static DB_PATH : OnceLock<String> = OnceLock::new();

pub fn initialize(config_path: &str) {
    DB_PATH.get_or_init(|| format!("{config_path}\\database.db"));
    let db_path = DB_PATH.get().unwrap();

    if !Path::new(&config_path).exists() {
        log::warn!("Created new config directory!");

        fs::create_dir_all(config_path).unwrap();
    }

    if !Path::new(&db_path).exists() {
        log::warn!("Created new database!");
    }

    get_connection().execute("
        CREATE TABLE IF NOT EXISTS accounts (
            id TEXT PRIMARY KEY,
            token TEXT NOT NULL,
            name TEXT NOT NULL,
            avatar_url TEXT NOT NULL
        )",
        ()
    ).unwrap();
}

pub fn get_connection() -> Connection {
    Connection::open(DB_PATH.get().unwrap()).unwrap()
}