// Used to validate input forms

use serenity::prelude::*;

#[tauri::command]
pub async fn validate_input(message: &str, validation_type: &str, validation_id: i8) -> Result<i8, i8> {
    match validation_type {
        "token" => {
            let result = validate_token(message).await;

            if result.is_ok() {
                return Ok(validation_id);
            }

            return Err(validation_id);
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
        .await?;

    Ok(())
}