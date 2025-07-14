use serenity::{all::CurrentUser, prelude::*};

use crate::{config::app_config, database::Database};

#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BotAccount {
    pub id: String,
    pub token: String,
    pub name: String,
    pub avatar_url: String,
}

impl Database {
    pub async fn validate_token(&self, token: &str) -> Result<(), String> {
        let _ = serenity::all::validate_token(token)
            .map_err(|err| err.to_string())?;

        let client = Client::builder(token, GatewayIntents::all())
            .status(serenity::all::OnlineStatus::Invisible)
            .await
            .map_err(|err| err.to_string())?;

        let _ = client
            .http
            .get_current_user()
            .await
            .map_err(|_| "The provided token was invalid")?;

        let bots = self.get_all_accounts().await?;
        for bot in bots {
            if bot.token == token {
                return Err("This bot is already added".into());
            }
        }

        Ok(())
    }

    pub async fn get_all_accounts(&self) -> Result<Vec<BotAccount>, String> {
        let conn = self.connection.lock().await;

        let mut stmt = conn
            .prepare("SELECT id, token, name, avatar_url FROM accounts")
            .map_err(|err| err.to_string())?;

        let result = stmt
            .query_map((), |row| {
                Ok(BotAccount {
                    id: row.get(0)?,
                    token: row.get(1)?,
                    name: row.get(2)?,
                    avatar_url: row.get(3)?,
                })
            })
            .map_err(|err| err.to_string())?
            .map(|row| row.unwrap())
            .collect();

        Ok(result)
    }

    pub async fn insert_account(&self, token: &str) -> Result<(), String> {
        let conn = self.connection.lock().await;
        let user = self.get_bot_info(token).await?;

        let mut stmt = conn
            .prepare("SELECT COUNT(*) FROM accounts WHERE token = ?1")
            .map_err(|err| err.to_string())?;

        let count: usize = stmt
            .query_row((token,), |row| row.get(0))
            .map_err(|err| err.to_string())?;

        if count > 0 {
            return Err("This Bot is already registered".to_string());
        }

        conn.execute(
            "
            INSERT INTO accounts (id, token, name, avatar_url)
            VALUES (?1, ?2, ?3, ?4)
            ",
            (&user.id, &user.token, &user.name, &user.avatar_url),
        )
        .map_err(|err| err.to_string())?;

        Ok(())
    }

    pub async fn update_account_token(&self, id: &str, new_token: &str) -> Result<(), String> {
        let conn = self.connection.lock().await;
        let user = self.get_bot_info(new_token).await?;

        if id != user.id {
            return Err("Token doesn't correspond to this bot".to_string());
        }

        conn.execute(
            "
            UPDATE accounts 
            SET token = ?1, name = ?2, avatar_url = ?3
            WHERE id = ?4
            ",
            (&user.token, &user.name, &user.avatar_url, &user.id),
        )
        .map_err(|err| err.to_string())?;

        Ok(())
    }

    pub async fn delete_account(&self, id: &str, delete_data: bool) -> Result<(), String> {
        let conn = self.connection.lock().await;

        conn.execute("DELETE FROM accounts WHERE id = ?1", (&id,))
            .map_err(|err| err.to_string())?;

        if !delete_data { return Ok(()) }

        let config_state = app_config();
        let path = &config_state.config_path;

        if std::fs::remove_file(path).is_err() {
            return Err("Failed to delete bot config file".to_string());
        }

        Ok(())
    }

    pub async fn update_account_info(&self, user: &CurrentUser) {
        let conn = self.connection.lock().await;

        conn.execute("
                UPDATE accounts
                SET name = ?1, avatar_url = ?2
                WHERE id = ?3
            ", (user.name.clone(), user.avatar_url().unwrap_or("".to_owned()), user.id.to_string())
        ).unwrap();
    }

    async fn get_bot_info(&self, token: &str) -> Result<BotAccount, String> {
        let client = Client::builder(token, GatewayIntents::all())
            .status(serenity::all::OnlineStatus::Invisible)
            .await
            .map_err(|err| err.to_string())?;

        let user = client
            .http
            .get_current_user()
            .await
            .map_err(|err| err.to_string())?;

        Ok(BotAccount {
            id: user.id.to_string(),
            token: token.to_string(),
            name: user.name.clone(),
            avatar_url: user.avatar_url().unwrap_or("".to_string()),
        })
    }
}
