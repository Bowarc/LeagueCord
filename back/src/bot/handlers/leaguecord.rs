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

        // TODO: helper functions to find channels and category (make sure found channels have the right category (leaguecord management))

        let id_cache = IdCache::new(ctx.clone(), guild.id).await.unwrap();

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
