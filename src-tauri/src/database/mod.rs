use rusqlite::Connection;
use std::sync::OnceLock;

use crate::logging::info;

use super::config::config_path;

pub mod bot_accounts;

static DB_PATH: OnceLock<String> = OnceLock::new();

pub(super) fn initialize() {
    DB_PATH.get_or_init(|| config_path("\\database.db"));

    get_connection()
        .execute(
            "
        CREATE TABLE IF NOT EXISTS accounts (
            id TEXT PRIMARY KEY,
            token TEXT NOT NULL,
            name TEXT NOT NULL,
            avatar_url TEXT NOT NULL
        )",
            (),
        )
        .unwrap();

    info!("{}", "Database Initialized");
}

pub(in crate::database) fn get_connection() -> Connection {
    Connection::open(DB_PATH.get().unwrap()).unwrap()
}
