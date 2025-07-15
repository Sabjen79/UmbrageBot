use strum_macros::Display;

#[derive(Display, Debug)]
pub enum NotifyEvents {
    #[strum(to_string = "BOT_LOGIN_SUCCESS")]
    BotLoginSuccess,

    #[strum(to_string = "BOT_SHUTDOWN_START")]
    BotShutdownStart,

    #[strum(to_string = "BOT_SHUTDOWN_SUCCESS")]
    BotShutdownSuccess,
}