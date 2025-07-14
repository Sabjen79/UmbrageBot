use serenity::all::EditProfile;

// #[tauri::command]
// pub async fn change_username(username: String) -> Result<(), String> {
//     let http = get_http();

//     let result = http
//         .get_current_user()
//         .await
//         .unwrap()
//         .edit(http, EditProfile::new().username(username))
//         .await;

//     match result {
//         Ok(()) => {}
//         Err(_) => {
//             return Err("Changing username too fast!".into())
//         }
//     }

//     Ok(())
// }