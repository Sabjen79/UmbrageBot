use std::time::{SystemTime, UNIX_EPOCH};


use serde::{Deserialize, Serialize};

use crate::{bot::{self, account_manager::activity::ActivityWrapper}, database::{self, Database}, logging::log_error};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DbActivity {
    id: String,
    type_num: u8,
    content: String,
    url: String
}

pub fn get_random() -> ActivityWrapper {
    let database = Database::get_state();

    let conn = database::connection();
    let bot_id = bot::bot_id();

    let mut indexes_lock = database.random_indexes.lock().unwrap();
    let indexes = indexes_lock.iter_mut()
        .find(|a| a.table_name.eq("activities")).unwrap();

    let count: Result<u8, _> = conn.query_one(
        "SELECT COUNT(*) FROM activities WHERE bot_id = ?1", 
        [bot_id], 
        |row| row.get(0)
    );

    match count {
        Ok(count) => {
            if count == 0 {
                return ActivityWrapper::None;
            }
        }
        Err(err) => {
            log_error!("Error getting count of activities: {}", err.to_string());
            return ActivityWrapper::None;
        }
    }

    let id = bot::bot_id();

    let result = conn.query_one("
            SELECT id, type_num, content, url
            FROM activities
            WHERE bot_id = ?1
            LIMIT 1
            OFFSET ?2
        ", 
        (id, indexes.take()),
        |row| {
            Ok(DbActivity { 
                id: row.get(0)?, 
                type_num: row.get(1)?, 
                content: row.get(2)?, 
                url: row.get(3)?
            })
        });

    match result {
        Ok(activity) => {
            match activity.type_num {
                0 => ActivityWrapper::None,
                1 => ActivityWrapper::Playing(activity.content),
                2 => ActivityWrapper::Streaming(activity.content, activity.url),
                3 => ActivityWrapper::Listening(activity.content),
                4 => ActivityWrapper::Watching(activity.content),
                5 => ActivityWrapper::Competing(activity.content),
                6 => ActivityWrapper::Custom(activity.content),
                _ => ActivityWrapper::None,
            }
        }
        Err(err) => {
            log_error!("Error getting random activity: {}", err.to_string());

            ActivityWrapper::None
        }
    }
}

pub fn get_all() -> Result<Vec<DbActivity>, String> {
    let conn = database::connection();
    let bot_id = bot::bot_id();

    let mut stmt = conn
        .prepare("
            SELECT id, type_num, content, url
            FROM activities
            WHERE bot_id = ?1
            ORDER BY id DESC
        ")
        .map_err(|err| err.to_string())?;

    let result = stmt
        .query_map([&bot_id], |row| {
            Ok(DbActivity { 
                id: row.get(0)?, 
                type_num: row.get(1)?, 
                content: row.get(2)?, 
                url: row.get(3)?
            })
        })
        .map_err(|err| err.to_string())?
        .map(|row| row.unwrap())
        .collect();

    Ok(result)
}

pub fn insert() -> Result<(), String> {
    let database = Database::get_state();
    let conn = database::connection();

    let mut indexes_lock = database.random_indexes.lock().unwrap();
    let indexes = indexes_lock.iter_mut()
        .find(|a| a.table_name.eq("activities")).unwrap();
    let bot_id = bot::bot_id();

    conn.execute(
        "
        INSERT INTO activities (id, bot_id, type_num, content, url) 
        VALUES (?1, ?2, ?3, ?4, ?5)
        ",
        (
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis().to_string(),
            bot_id,
            1,
            "Something",
            ""
        ),
    )
    .map_err(|err| err.to_string())?;

    indexes.generate();

    Ok(())
}

pub fn update(activity: DbActivity) -> Result<(), String> {
    let conn = database::connection();
    
    conn.execute(
        "
        UPDATE activities 
        SET type_num = ?1, content = ?2, url = ?3
        WHERE id = ?4
        ",
        (&activity.type_num, &activity.content, &activity.url, &activity.id),
    )
    .map_err(|err| err.to_string())?;

    Ok(())
}

pub fn delete(id: &str) -> Result<(), String> {
    let database = Database::get_state();
    let conn = database::connection();
    
    let mut indexes_lock = database.random_indexes.lock().unwrap();
    let indexes = indexes_lock.iter_mut()
        .find(|a| a.table_name.eq("activities")).unwrap();

    conn.execute("DELETE FROM activities WHERE id = ?1", [id])
        .map_err(|err| err.to_string())?;

    indexes.generate();

    Ok(())
}