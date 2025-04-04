pub struct Door;

#[serenity::async_trait]
impl serenity::all::EventHandler for Door {
    async fn message(&self, ctx: serenity::all::Context, message: serenity::all::Message) {
        super::module_command(&ctx, "Door", message).await;
    }

    async fn guild_member_addition(
        &self,
        ctx: serenity::all::Context,
        new_member: serenity::model::prelude::Member,
    ) {
        use {
            crate::data::LeagueCordData,
            serenity::all::{CacheHttp as _, CreateMessage, Mentionable as _},
        };

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
                warn!(
                    "User {} tried to join with an invite that did not correspond to any group.",
                    new_member.user.name
                );

                if let Err(e) = data
                    .ids
                    .bot_log_channel
                    .send_message(
                        ctx.http(),
                        CreateMessage::new().content(format!(
                            "User {} tried to join with an invite that did not correspond to any group.",
                            new_member.user.name
                        )),
                    )
                    .await
                {
                    error!("Failed to send error message to log channel due to: {e}")
                }

                if let Err(e) = new_member
                    .kick_with_reason(ctx.http(), "Not appart of a valid group")
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
            if let Err(e) = new_member.add_role(ctx.http(), group.role).await {
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
            let http = ctx.http();

            if let Err(e) = group_text_channel_id
                .send_message(
                    http,
                    CreateMessage::new().content(format!(
                        "New player joined: {}\nMake sure to use `!help` if you have any question",
                        new_member.mention()
                    )),
                )
                .await
            {
                error!("Failed to send welcome message due to: {e}");
            }

            saved_invites_lock.set(invite.code.clone(), invite.uses);
        }
        // Found none or multiple invites that changed since last check
        else {
            // TODO: ? lost user channel ?
            warn!(
                "Failed to find what invite user {}({}) used to join the server",
                new_member.user.name, new_member.user.id
            );
            if let Err(e) = data
                .ids
                .bot_log_channel
                .send_message(
                    ctx.http(),
                    CreateMessage::new().content(format!(
                        "User {} tried to join with an invite that did not correspond to any group.",
                        new_member.user.name
                    )),
                )
                .await
            {
                error!("Failed to send error message to log channel due to: {e}")
            }

            if let Err(e) = new_member
                .kick_with_reason(ctx.http(), "Could not find the invite the user joined with")
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
                if let Err(e) = new_member.user.dm(ctx.http(), CreateMessage::new().content(format!("Hi user {}\nI was not able to find what group you joined, please retry to join the server using the appropriate invite link\nIf this issue persists, please contact `Bowarc`", new_member.user.id))).await {
                    super::log_error(ctx.clone(), &data.ids, &format!(
                        "Failed dm {}({}) due to {e}",
                        new_member.user.name, new_member.user.id
                    )).await;
                }
            }

            saved_invites_lock
                .update(ctx.http, &data.ids)
                .await
                .unwrap();
        }
    }

    async fn guild_member_removal(
        &self,
        ctx: serenity::all::Context,
        _guild_id: serenity::all::GuildId,
        user: serenity::all::User,
        _member_data_if_available: Option<serenity::all::Member>,
    ) {
        use crate::data::LeagueCordData;
        // Get a read ref of the data
        let ctx_data_storage = ctx.data.clone();
        let ctx_data_storage_read = ctx_data_storage.read().await;
        let Some(data) = ctx_data_storage_read.get::<LeagueCordData>() else {
            error!("Could not get tracked invites from data");
            return;
        };

        // Lock groups
        let mut groups = data.groups.write().await;

        // lock invites
        let mut invites = data.invites.write().await;

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
                invites.rm(&group.invite_code);
                groups.remove(i);
                continue;
            }

            i += 1
        }
    }
}
