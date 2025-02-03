// Used to validate input forms

use serde::Serialize;
use serenity::prelude::*;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InputValidationError {
    validation_id: i8,
    message: String
}

#[tauri::command]
pub async fn validate_input(message: &str, validation_type: &str, validation_id: i8) -> Result<i8, InputValidationError> {
    match validation_type {
        "token" => {
            let result = validate_token(message).await;

            if result.is_ok() {
                return Ok(validation_id);
            }

            return Err(InputValidationError {
                validation_id: validation_id,
                message: result.unwrap_err().to_string()
            })
        },
        _ => {
            Ok(validation_id)
        }
    }
}

async fn validate_token(token: &str) -> Result<(), Box<dyn std::error::Error>> {
    let _ = serenity::all::validate_token(token)?;

    let client = Client::builder(token, GatewayIntents::all())
        .status(serenity::all::OnlineStatus::Invisible)
        .await?;

    let _ = client
        .http
        .get_current_user()
        .await
        .map_err(|_| "The provided token does not correspond to a bot")?;

    let bots = crate::database::bot_accounts::get_all_accounts()?;
    for bot in bots {
        if bot.token == token {
            return Err("This bot is already added".into());
        }
    }

    Ok(())
}