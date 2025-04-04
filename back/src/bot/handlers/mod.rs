// mod basic;
mod debug;
mod leaguecord;
// mod logger;
mod door;
mod player_helper;
mod purge;

// pub use basic::Basic;
pub use debug::Debug;
pub use leaguecord::LeagueCord;
// pub use logger::Logger;
pub use door::Door;
pub use player_helper::PlayerHelper;
pub use purge::Purge;

use crate::data::IdCache;

pub async fn log_error(ctx: serenity::all::Context, ids: &IdCache, message: &str) {
    use serenity::all::CreateMessage;

    error!("{message}");

    if let Err(e) = ids
        .bot_log_channel
        .send_message(ctx.http, CreateMessage::new().content(message))
        .await
    {
        error!("Failed to send error message to log channel due to: {e}")
    }
}

pub async fn module_command(
    ctx: &serenity::all::Context,
    module_name: &str,
    message: serenity::all::Message,
) {
    use {crate::bot::command, serenity::all::CacheHttp as _};

    let Some(_args) = command::parse(
        &message,
        "modules",
        command::Case::Insensitive,
        command::Prefix::Yes,
    ) else {
        return;
    };

    message
        .channel_id
        .say(ctx.http(), format!("{module_name} module is loaded !"))
        .await
        .unwrap();
}
