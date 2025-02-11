use serenity::{
    all::{Context, EditMember, EditRole, EventHandler, GuildId, Message},
    prelude::TypeMapKey,
};

pub struct LeagueCord;

pub struct IdCache {
    guild: GuildId,
}

#[derive(Debug)]
pub struct TrackedInvites{
    inner: std::collections::HashMap<String, u64> 
}

impl TypeMapKey for IdCache {
    type Value = Self;
}

impl TypeMapKey for TrackedInvites {
    type Value = Self;
}

impl std::ops::Deref for TrackedInvites{
    type Target = std::collections::HashMap<String, u64>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[serenity::async_trait]
impl EventHandler for LeagueCord {
    async fn ready(&self, ctx: Context, data_about_bot: serenity::model::prelude::Ready) {
        if data_about_bot.guilds.len() != 1 {
            error!("Expected to live in only one server");
            std::process::exit(1);
        }

        let id_cache = IdCache {
            guild: data_about_bot.guilds.first().unwrap().id,
        };

        ctx.data.write().await.insert::<IdCache>(id_cache);

        debug!("Bot is loaded")
    }

    async fn guild_member_addition(
        &self,
        ctx: Context,
        mut new_member: serenity::model::prelude::Member,
    ) {
        let data_read  = ctx.data.read().await;
        let Some(saved_invites) = data_read.get::<TrackedInvites>() else{
            error!("Could not get tracked invites from data");
            return;
        };
        // get saved invites from saved data

        let invites = ctx
            .http
            .get_guild_invites(new_member.guild_id)
            .await
            .unwrap();

        for invite in invites.iter() {
            let Some(saved_invite_use_count) = saved_invites.get(&invite.code) else {
                println!("New invite: {invite:?}");
                continue;
            };

            if invite.uses == *saved_invite_use_count {
                continue;
            }
            debug!("{saved_invites:?}");

            info!("Found invite: {invite:?}");
            let guild = ctx
                .clone()
                .http
                .get_guild(new_member.guild_id)
                .await
                .unwrap();

            let role = if let Some(role) = guild
                .roles
                .values()
                .find(|role| role.name == invite.channel.name)
            {
                role.clone()
            } else {
                guild
                    .create_role(
                        ctx.clone(),
                        EditRole::new().name(invite.channel.name.clone()),
                    )
                    .await
                    .unwrap()
            };

            new_member
                .edit(ctx.clone(), EditMember::new().roles(vec![role.id]))
                .await
                .unwrap();
        }
    }

    async fn message(&self, ctx: Context, message: Message) {
        debug!("Message: {message:?}")
    }
}
