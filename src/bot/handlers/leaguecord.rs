use std::time::Duration;

use serenity::all::{CreateChannel, CreateMessage, Mentionable};

use {
    serenity::{
        all::{Context, EditMember, EditRole, EventHandler, GuildId, Message},
        prelude::TypeMapKey,
    },
    std::{collections::HashMap, sync::Arc},
    tokio::sync::RwLock,
};

pub mod data;

pub struct LeagueCord;

#[derive(Debug, Clone)]
pub struct LeagueCordData {
    pub ids: Arc<data::IdCache>,
    pub invites: Arc<RwLock<HashMap<data::InviteCode, data::InviteUseCount>>>,
    pub groups: Arc<RwLock<Vec<data::Group>>>,
}

impl TypeMapKey for LeagueCordData {
    type Value = Self;
}

pub async fn build_invite_list(
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

        let guild = data_about_bot.guilds.first().unwrap();

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

        let id_cache = data::IdCache {
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

        let invites = build_invite_list(ctx.clone(), &id_cache)
            .await
            .unwrap_or_default();

        let data = LeagueCordData {
            ids: Arc::new(id_cache),
            invites: Arc::new(RwLock::new(invites)),
            groups: Arc::new(RwLock::new(Vec::new())),
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
            let invite = used_invites.first().unwrap();

            let groups = data.groups.write().await;

            let Some(group) = groups.iter().find(|group| group.invite_code == invite.code) else {
                if let Err(e) = new_member
                    .kick_with_reason(ctx.http.clone(), "Not appart of a valid group")
                    .await
                {
                    error!(
                        "Failed to kick new member '{}'({}) due to: {e}",
                        new_member.display_name(),
                        new_member.user.id
                    );
                    let _ignored = data
                        .ids
                        .bot_log_channel
                        .send_message(
                            ctx.http.clone(),
                            CreateMessage::new().content(format!(
                                "Failed to kick new member: '{}'({}) due to {e}",
                                new_member.display_name(),
                                new_member.user.id
                            )),
                        )
                        .await;
                };
                return;
            };
            if let Err(e) = new_member.add_role(ctx.http.clone(), group.role).await {
                error!(
                    "Failed to set group role for new member: '{}'({}) due to: {e}",
                    new_member.display_name(),
                    new_member.user.id
                );

                let _ignored = data
                    .ids
                    .bot_log_channel
                    .send_message(
                        ctx.http.clone(),
                        CreateMessage::new().content(format!(
                            "Failed to set group role for new member: '{}'({}) due to {e}",
                            new_member.display_name(),
                            new_member.user.id
                        )),
                    )
                    .await;
            }

            debug!(
                "Successfully moved new member ({}) to group: {}",
                new_member.user.id, group.invite_code
            );

            let group_text_channel_id = group.text_channel;
            let http = ctx.http.clone();

            tokio::task::spawn(async move {
                tokio::time::sleep(Duration::from_secs(5)).await;
                if let Err(e) = group_text_channel_id
                    .send_message(
                        http,
                        CreateMessage::new().content(format!("New player joined: {}\nMake sure to use `!help` if you have any question", new_member.mention())),
                    )
                    .await
                {
                    error!("Failed to send welcome message due to: {e}");
                }
            });
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
        use crate::bot::command;
        'help: {
            let Some(args) = command::parse(&message, "help", command::Case::Insensitive , command::Prefix::Yes) else{
                break 'help;
            };

            message.reply(ctx.http, "Temporary help message").await.unwrap();
        }
        
        // debug!("Message: {message:?}")
    }
}

