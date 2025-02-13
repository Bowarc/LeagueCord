use std::{collections::HashMap, sync::Arc};

use serenity::{
    all::{Context, EditMember, EditRole, EventHandler, GuildId, Message},
    prelude::TypeMapKey,
};
use tokio::sync::RwLock;

mod data;

pub struct LeagueCord;

#[derive(Debug, Clone)]
pub struct LeagueCordData {
    ids: Arc<data::IdCache>,
    invites: Arc<RwLock<HashMap<data::InviteCode, data::InviteUseCount>>>,
    groups: Arc<RwLock<Vec<data::Group>>>
}

impl TypeMapKey for LeagueCordData {
    type Value = Self;
}

async fn build_invite_list(
    ctx: Context,
    ids: &data::IdCache,
) -> Result<HashMap<data::InviteCode, data::InviteUseCount>, String> {
    let Ok(invite_list) = ctx.http.get_guild_invites(ids.guild).await else {
        return Err(format!(
            "Could not get the invite list for guild: {:?}",
            ids.guild
        ));
    };

    let out = invite_list
        .into_iter()
        .map(|invite| (invite.code, invite.uses))
        .collect();

    Ok(out)
}

#[serenity::async_trait]
impl EventHandler for LeagueCord {
    async fn ready(&self, ctx: Context, data_about_bot: serenity::model::prelude::Ready) {
        if data_about_bot.guilds.len() != 1 {
            error!("Expected to live in only one server");
            std::process::exit(1);
        }

        let id_cache = data::IdCache {
            guild: data_about_bot.guilds.first().unwrap().id,
        };

        let invites = build_invite_list(ctx.clone(), &id_cache).await.unwrap();

        let data = LeagueCordData {
            ids: Arc::new(id_cache),
            invites: Arc::new(RwLock::new(invites)),
            groups: Arc::new(RwLock::new(Vec::new()))
        };

        ctx.data.write().await.insert::<LeagueCordData>(data);

        debug!("Bot is loaded")
    }

    async fn guild_member_addition(
        &self,
        ctx: Context,
        mut new_member: serenity::model::prelude::Member,
    ) {
        let ctx_data_storage = ctx.data.clone();
        let ctx_data_storage_read = ctx_data_storage.read().await;
        let Some(data) = ctx_data_storage_read.get::<LeagueCordData>() else {
            error!("Could not get tracked invites from data");
            return;
        };

        let mut saved_invites_lock = data.invites.write().await;
        // get saved invites from saved data

        let invites = ctx
            .http
            .get_guild_invites(new_member.guild_id)
            .await
            .unwrap();

        // Can we find in the guild invite list, an invite where the use count is different that what we saved ?
        // If there is a lot of pple that join at the same time, this might return multiple results.
        // For that we can send the user to a special channel where we can ask for the invite code directly

        let used_invites = invites
            .iter()
            .filter(|invite| {
                let Some(saved_invite_use_count) = saved_invites_lock.get(&invite.code) else {
                    println!("New invite: {invite:?}");
                    return false;
                };

                invite.uses != *saved_invite_use_count
            })
            .collect::<Vec<_>>();

        if used_invites.len() == 1 {
            let _invite = used_invites.first().unwrap();

            // TODO:  Query the group based on the invite code, get the role id, assign the role to the user

            /*

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

            */
        } else if used_invites.is_empty() {
            // ?? The used invite was not registered, this must be an old one
            // Probably kick the user or send them to a specific channel
        } else {
            // multiple matches
            // Send them to a channel that request them to send the invite link or the group code idfk
        }

        // Force update the invite list

        *saved_invites_lock = build_invite_list(ctx, &data.ids).await.unwrap()
    }

    async fn message(&self, ctx: Context, message: Message) {
        debug!("Message: {message:?}")
    }
}
