use std::sync::OnceLock;
use rusqlite::Connection;

use crate::config;

pub mod bot_accounts;

static DB_PATH : OnceLock<String> = OnceLock::new();

pub fn initialize() {
    DB_PATH.get_or_init(|| config::config_path("\\database.db"));

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