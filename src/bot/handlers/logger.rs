use serenity::all::{ChannelId, Context, EventHandler, Guild, GuildId, Message, MessageId, Ready};

pub struct Logger;

#[serenity::async_trait]
impl EventHandler for Logger {
    async fn ready(&self, ctx: Context, data_about_bot: Ready) {
        debug!("Bot is ready");
        let iter = data_about_bot.guilds.iter();

        let guilds = futures::future::join_all(iter.map(|g| g.id).map(|id| {
            let http = ctx.http.clone();
            async move { Guild::get(http.clone(), id).await }
        }))
        .await
        .into_iter()
        .flatten()
        .map(|guild| guild.name)
        .collect::<Vec<String>>();

        debug!(
            "The bot is connected as '{}'({}) to the following guilds: {guilds:#?}",
            data_about_bot.user.name, data_about_bot.user.id
        );
    }
    async fn message(&self, ctx: Context, message: Message) {
        let guild_name = 'guild_name: {
            let Some(id) = message.guild_id else {
                break 'guild_name String::from("Unnamed guild");
            };

            let Ok(guild) = Guild::get(&ctx.http, id).await else {
                break 'guild_name String::from("Unnamed guild");
            };

            guild.name
        };

        trace!(
            "{} sent '{}' in {guild_name}",
            message.author.name, message.content
        );
    }
}
