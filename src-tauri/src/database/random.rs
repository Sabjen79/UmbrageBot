use rand::seq::SliceRandom;
use rusqlite::params;

use crate::{database, logging::log_error};


pub(in crate::database)
struct RandomIndexGenerator {
    table_name: String,
    bot_id: String,
    list: Vec<u16>
}

impl RandomIndexGenerator {
    pub fn new(table_name: &str, bot_id: &String) -> Self {
        Self {
            table_name: table_name.to_string(),
            bot_id: bot_id.clone(),
            list: Vec::new()
        }
    }

    pub fn generate(&mut self) {
        let sql = format!("SELECT COUNT(*) FROM {} WHERE bot_id = {}", self.table_name, self.bot_id);
        let conn = database::connection();
        let mut stmt = conn.prepare(sql.as_str()).unwrap();

        let query = stmt.query_one(params![], |row| {
            Ok(row.get(0)?)
        });

        match query {
            Ok(count) => {
                self.list = (0..count).collect();
                self.list.shuffle(&mut rand::rng());

                println!("{:?}", self.list);
            }
            Err(err) => {
                log_error!("Cannot generate indexes for table '{}': {}", self.table_name, err.to_string());
            }
        }
    }

    pub fn take(&mut self) -> u16{
        match self.list.pop() {
            Some(value) => {
                if self.list.is_empty() {
                    self.generate();
                }

                value
            }
            None => {
                panic!("Cannot run 'take' function before 'generate'");
            }
        }
    }
}