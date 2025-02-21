use crate::data::InviteTracker;

use {
    crate::data::{IdCache, LeagueCordData},
    serenity::all::{Context, EventHandler, Message},
    serenity::all::{
        CreateChannel, CreateCommand, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter,
        CreateInteractionResponse, CreateInteractionResponseMessage, CreateMessage, GuildId,
        Interaction, Member, Mentionable, User,
    },
    std::sync::Arc,
    std::time::Duration,
    tokio::sync::RwLock,
};

pub struct LeagueCord;

#[serenity::async_trait]
impl EventHandler for LeagueCord {
    async fn ready(&self, ctx: Context, data_about_bot: serenity::model::prelude::Ready) {
        // if data_about_bot.guilds.len() != 1 {
        //     error!("Expected to live in only one server");
        //     std::process::exit(1);
        // }

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

    async fn guild_member_addition(
        &self,
        ctx: Context,
        new_member: serenity::model::prelude::Member,
    ) {
        // Get a read ref of the data
        let ctx_data_storage = ctx.data.clone();
        let ctx_data_storage_read = ctx_data_storage.read().await;
        let Some(data) = ctx_data_storage_read.get::<LeagueCordData>() else {
            error!("Could not get tracked invites from data");
            return;
        };

        let mut saved_invites_lock = data.invites.write().await;

        // Query server invites
        let server_invites = ctx
            .http
            .get_guild_invites(new_member.guild_id)
            .await
            .unwrap();

        // Can we find in the guild invite list, an invite where the use count is different that what we saved ?
        // If there is a lot of pple that join at the same time, this might return multiple results.
        // For that we can send the user to a special channel where we can ask for the invite code directly

        // Get the invites that have a different use count that what was saved
        let used_invites = server_invites
            .iter()
            .filter(|invite| {
                let Some(saved_invite_use_count) = saved_invites_lock.get(&invite.code) else {
                    debug!("New invite: {invite:?}");
                    return false;
                };

                invite.uses != *saved_invite_use_count
            })
            .collect::<Vec<_>>();

        // Only one invite changed
        if used_invites.len() == 1 {
            let invite = used_invites.first().unwrap();

            let mut groups = data.groups.write().await;

            // Find the group associated to that invite, else kick em
            let Some(group) = groups
                .iter_mut()
                .find(|group| group.invite_code == invite.code)
            else {
                warn!("User {} tried to join with an invite that did not correspond to any group.", new_member.user.name);
                if let Err(e) = new_member
                    .kick_with_reason(ctx.http.clone(), "Not appart of a valid group")
                    .await
                {
                    super::log_error(
                        ctx.clone(),
                        &data.ids,
                        &format!(
                            "Failed to kick new member '{}'({}) due to: {e}",
                            new_member.display_name(),
                            new_member.user.id
                        ),
                    )
                    .await
                };
                return;
            };
            if let Err(e) = new_member.add_role(ctx.http.clone(), group.role).await {
                super::log_error(
                    ctx.clone(),
                    &data.ids,
                    &format!(
                        "Failed to set group role for new member: '{}'({}) due to: {e}",
                        new_member.display_name(),
                        new_member.user.id
                    ),
                )
                .await
            }
            group.users.push(new_member.user.id);

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
        }
        // Could not find what invite was used to join the server
        else if used_invites.is_empty() {
            // TODO: better management, maybe a lost users channel / server ?
            warn!(
                "Failed to find what invite user {}({}) used to join the server",
                new_member.user.name, new_member.user.id
            );
            if let Err(e) = new_member
                .kick_with_reason(
                    ctx.http.clone(),
                    "Could not find the invite the user joined with",
                )
                .await
            {
                super::log_error(
                    ctx.clone(),
                    &data.ids,
                    &format!(
                        "Failed to kick {}({}) due to {e}",
                        new_member.user.name, new_member.user.id
                    ),
                )
                .await;
            }
            // TODO: Better user message.
            {
                if let Err(e) = new_member.user.dm(ctx.http.clone(), CreateMessage::new().content(format!("Hi user {}\nI was not able to find what group you joined, please retry to join the server using the appropriate invite link\nIf this issue persists, please contact `Bowarc`", new_member.user.id))).await {
                    super::log_error(ctx.clone(), &data.ids, &format!(
                        "Failed dm {}({}) due to {e}",
                        new_member.user.name, new_member.user.id
                    )).await;
                }
            }
        }
        // Found multiple invites that changed since last check
        else {
            // TODO: ? lost user channel ?
            warn!(
                "Failed to find what invite user {}({}) used to join the server",
                new_member.user.name, new_member.user.id
            );
            if let Err(e) = new_member
                .kick_with_reason(
                    ctx.http.clone(),
                    "Could not find the invite the user joined with",
                )
                .await
            {
                super::log_error(
                    ctx.clone(),
                    &data.ids,
                    &format!(
                        "Failed to kick {}({}) due to {e}",
                        new_member.user.name, new_member.user.id
                    ),
                )
                .await;
            }
            // TODO: Better user message.
            {
                if let Err(e) = new_member.user.dm(ctx.http.clone(), CreateMessage::new().content(format!("Hi user {}\nI was not able to find what group you joined, please retry to join the server using the appropriate invite link\nIf this issue persists, please contact `Bowarc`", new_member.user.id))).await {
                    super::log_error(ctx.clone(), &data.ids, &format!(
                        "Failed dm {}({}) due to {e}",
                        new_member.user.name, new_member.user.id
                    )).await;
                }
            }
            // multiple matches
            // Send them to a channel that request them to send the invite link or the group code idfk
        }

        // Force update the invite list

        saved_invites_lock
            .update(ctx.http, &data.ids)
            .await
            .unwrap();
    }

    async fn guild_member_removal(
        &self,
        ctx: Context,
        _guild_id: GuildId,
        user: User,
        _member_data_if_available: Option<Member>,
    ) {
        // Get a read ref of the data
        let ctx_data_storage = ctx.data.clone();
        let ctx_data_storage_read = ctx_data_storage.read().await;
        let Some(data) = ctx_data_storage_read.get::<LeagueCordData>() else {
            error!("Could not get tracked invites from data");
            return;
        };

        // Lock groups
        let mut groups = data.groups.write().await;

        if let Some(group) = groups.iter_mut().find(|g| g.users.contains(&user.id)) {
            group.users.retain(|id| id != &user.id);
        }

        let mut i = 0;

        loop {
            let Some(group) = groups.get(i) else {
                break;
            };

            if group.users.is_empty() {
                group.cleanup_for_deletion(ctx.clone(), &data.ids).await;
                debug!("Removing empty group: {}", group.invite_code);
                groups.remove(i);
                continue;
            }

            i += 1
        }
    }

    async fn message(&self, ctx: Context, message: Message) {
        use crate::bot::command;
        'help: {
            let Some(_args) = command::parse(
                &message,
                "help",
                command::Case::Insensitive,
                command::Prefix::Yes,
            ) else {
                break 'help;
            };

            message
                .reply(ctx.http, "Temporary help message")
                .await
                .unwrap();
        }

        // debug!("Message: {message:?}")
    }
    async fn interaction_create(&self, ctx: Context, i: serenity::all::Interaction) {
        let Interaction::Command(c) = i else {
            return;
        };

        for option in c.data.options.iter() {
            println!("{option:?}");
        }

        if let Err(e) = c
            .create_response(
                ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("Content")
                        .embed(
                            CreateEmbed::new()
                                .author(CreateEmbedAuthor::new("Me"))
                                .title("This is a title")
                                .description("Simple description")
                                .footer(CreateEmbedFooter::new("Footer")),
                        )
                        .ephemeral(true),
                ),
            )
            .await
        {
            error!(
                "Failed to send a reponse to command {} due to: {e}",
                c.data.name
            )
        }
    }
}
