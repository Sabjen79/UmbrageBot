use rusqlite::Connection;
use std::sync::OnceLock;

use crate::logging::log_info;

use super::config::config_path;

pub mod bot_accounts;

static DB_PATH: OnceLock<String> = OnceLock::new();

pub(super) fn initialize() {
    DB_PATH.set(config_path("\\database.db")).unwrap();

    // REMINDER: Clear every table accordingly when deleting bots
    get_connection()
        .execute("
            CREATE TABLE IF NOT EXISTS accounts (
                id TEXT PRIMARY KEY,
                token TEXT NOT NULL,
                name TEXT NOT NULL,
                avatar_url TEXT NOT NULL
            )", ())
        .unwrap();

    get_connection()
        .execute("
            CREATE TABLE IF NOT EXISTS usernames (
                id TEXT PRIMARY KEY,
                bot_id TEXT NOT NULL,
                date_created INTEGER,
                username TEXT NOT NULL
            )", ())
        .unwrap();

    log_info!("{}", "Database Initialized");
}

pub(in crate::database) fn get_connection() -> Connection {
    Connection::open(DB_PATH.get().unwrap()).unwrap()
}
