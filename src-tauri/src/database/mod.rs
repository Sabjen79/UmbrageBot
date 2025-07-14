use rusqlite::Connection;
use tauri::{Manager, State};
use tokio::sync::Mutex;

use crate::{app_handle, config::app_config, logging::log_info};

pub mod bot_accounts;
pub mod commands;

/// Manager responsible for database operations
pub struct Database {
    pub(in crate::database) connection: Mutex<Connection>
}

pub fn database() -> State<'static, Database> {
    app_handle().state::<Database>()
}

impl Database {
    pub async fn new() -> Database {
        let config = app_config();

        let path = config.config_path.clone() + "\\database.db";

        let _self = Self { 
            connection: Mutex::new(Connection::open(&path).unwrap())
        };

        let conn = Connection::open(&path).unwrap();

        // REMINDER: Clear every table accordingly when deleting bots
        conn.execute("
                CREATE TABLE IF NOT EXISTS accounts (
                    id TEXT PRIMARY KEY,
                    token TEXT NOT NULL,
                    name TEXT NOT NULL,
                    avatar_url TEXT NOT NULL
                )", ())
            .unwrap();

        log_info!("{}", "Database Initialized");

        return _self;
    }
}
