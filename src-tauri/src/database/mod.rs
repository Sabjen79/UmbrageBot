use std::{collections::HashMap, sync::Arc};

use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use tauri::{Manager, State};
use tokio::sync::Mutex;

use crate::{app_config::{self}, app_handle, database::random::RandomIndexGenerator, logging::log_info};

pub mod bot_accounts;
pub mod activities;
pub mod commands;
pub mod random;

/// Manager responsible for database operations
pub struct Database {
    connection_pool: Arc<Pool<SqliteConnectionManager>>,
    random_indexes: Mutex<HashMap<String, RandomIndexGenerator>>
}

const CREATE_DB_SQL: &'static str = include_str!("scripts/create_db.sql");

impl Database {
    pub async fn new() -> Database {
        let path = app_config::config_path() + "\\database.db";

        let manager = SqliteConnectionManager::file(path);

        let _self = Self {
            connection_pool: Arc::new(r2d2::Pool::new(manager).unwrap()),
            random_indexes: Mutex::new(HashMap::new())
        };

        let conn = _self.connection_pool.clone();

        conn.get().unwrap().execute_batch(CREATE_DB_SQL).unwrap();

        log_info!("{}", "Database Initialized");

        return _self;
    }

    pub(in crate::database)
    fn get_state() -> State<'static, Database> {
        app_handle().state::<Database>()
    }
}

pub fn connection() -> PooledConnection<SqliteConnectionManager> {
    let db = Database::get_state();

    db.connection_pool.clone().get().unwrap()
}

pub async fn create_indexes(bot_id: &String) {
    let db = Database::get_state();

    let mut list = db.random_indexes.lock().await;

    list.clear();

    list.insert("activities".to_string(), RandomIndexGenerator::new("activities", bot_id));

    list.iter_mut().for_each(|f| f.1.generate());
}