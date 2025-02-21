use super::get_connection;
use serenity::prelude::*;

#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BotAccount {
    pub id: String,
    pub token: String,
    pub name: String,
    pub avatar_url: String,
}

#[tauri::command]
pub async fn validate_token(token: &str) -> Result<(), String> {
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

    let bots = get_all_accounts()?;
    for bot in bots {
        if bot.token == token {
            return Err("This bot is already added".into());
        }
    }

    Ok(())
}

#[tauri::command]
pub fn get_all_accounts() -> Result<Vec<BotAccount>, String> {
    let conn = get_connection();

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

#[tauri::command]
pub async fn insert_account(token: &str) -> Result<(), String> {
    let conn = get_connection();
    let user = get_bot_info(token).await?;

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

#[tauri::command]
pub async fn update_account_token(id: &str, new_token: &str) -> Result<(), String> {
    let conn = get_connection();
    let user = get_bot_info(new_token).await?;

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

#[tauri::command]
pub fn delete_account(id: &str, delete_data: bool) -> Result<(), String> {
    let conn = get_connection();

    conn.execute("DELETE FROM accounts WHERE id = ?1", (&id,))
        .map_err(|err| err.to_string())?;

    if !delete_data { return Ok(()) }

    let path = crate::config::config_path(format!("\\config\\{}.json", id).as_str());

    if std::fs::remove_file(&path).is_err() {
        return Err("Failed to delete bot config file".to_string());
    }

    conn.execute("DELETE FROM usernames WHERE bot_id = ?1", (&id,))
        .map_err(|err| err.to_string())?;

    Ok(())
}

// pub fn update_account_info(user: &CurrentUserRef) {
//     let conn = database::get_connection();

//     conn.execute("
//             UPDATE accounts
//             SET name = ?1, avatar_url = ?2
//             WHERE id = ?3
//         ", (user.name.clone(), user.avatar_url().unwrap_or("".to_owned()), user.id.to_string())
//     ).unwrap();
// }

pub async fn get_bot_info(token: &str) -> Result<BotAccount, String> {
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
