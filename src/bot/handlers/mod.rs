// mod basic;
mod debug;
mod leaguecord;
// mod logger;
mod purge;

// pub use basic::Basic;
pub use debug::Debug;
pub use leaguecord::LeagueCord;
// pub use logger::Logger;
pub use purge::Purge;

pub async fn log_error(
    ctx: serenity::all::Context,
    ids: &leaguecord::data::IdCache,
    message: &str,
) {
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
