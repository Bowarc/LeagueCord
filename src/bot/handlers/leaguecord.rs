pub struct LeagueCord;

#[serenity::async_trait]
impl serenity::all::EventHandler for LeagueCord {
    async fn ready(
        &self,
        ctx: serenity::all::Context,
        data_about_bot: serenity::model::prelude::Ready,
    ) {
        use {
            crate::data::{IdCache, InviteTracker, LeagueCordData},
            serenity::all::{CreateChannel, CreateCommand},
            std::sync::Arc,
            tokio::sync::RwLock,
        };

        if data_about_bot.guilds.len() != 1 {
            error!("Expected to live in only one server");
            std::process::exit(1);
        }

        let guild = ctx
            .http
            .get_guild(data_about_bot.guilds.first().unwrap().id)
            .await
            .unwrap();

        // TESTING COMMANDS
        // TODO: REMOVE THIS
        {
            guild
                .create_command(
                    ctx.http.clone(),
                    CreateCommand::new("test").description("Test command"),
                )
                .await
                .unwrap();
        }

        let graveyard_category = match ctx
            .http
            .get_channels(guild.id)
            .await
            .unwrap()
            .iter()
            .find(|channel| channel.name == "graveyard")
        {
            Some(channel) => channel.id,
            None => {
                guild
                    .id
                    .create_channel(
                        ctx.http.clone(),
                        CreateChannel::new("graveyard").kind(serenity::all::ChannelType::Category),
                    )
                    .await
                    .unwrap()
                    .id
            }
        };

        let bot_log_channel = match ctx
            .http
            .get_channels(guild.id)
            .await
            .unwrap()
            .iter()
            .find(|channel| channel.name == "bot_logs")
        {
            Some(channel) => channel.id,
            None => {
                guild
                    .id
                    .create_channel(
                        ctx.http.clone(),
                        CreateChannel::new("bot_logs").kind(serenity::all::ChannelType::Text),
                    )
                    .await
                    .unwrap()
                    .id
            }
        };

        let id_cache = IdCache {
            guild: guild.id,
            admin_role: ctx
                .http
                .get_guild_roles(guild.id)
                .await
                .unwrap()
                .iter()
                .find(|role| role.name == "-")
                .unwrap()
                .id,
            graveyard_category,
            bot_log_channel,
        };

        let invites = InviteTracker::new(ctx.http, &id_cache).await.unwrap();

        let data = LeagueCordData {
            ids: Arc::new(id_cache),
            invites: Arc::new(RwLock::new(invites)),
            groups: Arc::new(RwLock::new(Vec::new())),
        };

        ctx.data.write().await.insert::<LeagueCordData>(data);

        debug!("Bot is loaded")
    }


    async fn message(&self, ctx: serenity::all::Context, message: serenity::all::Message) {
        super::module_command(&ctx, "LeagueCord(main)", message).await

        // debug!("Message: {message:?}")
    }
    
}
