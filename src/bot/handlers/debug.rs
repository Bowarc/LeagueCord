use serenity::all::EditChannel;

use serenity::all::{Context, EventHandler, Message};

pub struct Debug;

#[serenity::async_trait]
impl EventHandler for Debug {
    async fn message(&self, ctx: Context, message: Message) {
        let ctx_data_storage = ctx.data.clone();
        let ctx_data_storage_read = ctx_data_storage.read().await;
        let Some(data) = ctx_data_storage_read.get::<super::leaguecord::LeagueCordData>() else {
            error!("Could not get tracked invites from data");
            return;
        };

        if !message
            .author
            .has_role(ctx.http.clone(), data.ids.guild, data.ids.admin_role)
            .await
            .unwrap()
        {
            return;
        }
        create_group(ctx.clone(), &message).await;
        cleanup(ctx, &message).await
    }
}

async fn create_group(ctx: Context, message: &Message) {
    use crate::bot::command;
    let Some(_args) = command::parse(
        message,
        "cg",
        command::Case::Insensitive,
        command::Prefix::Yes,
    ) else {
        return;
    };

    let ctx_data_storage = ctx.data.clone();
    let ctx_data_storage_read = ctx_data_storage.read().await;
    let Some(data) = ctx_data_storage_read.get::<super::leaguecord::LeagueCordData>() else {
        error!("Could not get tracked invites from data");
        return;
    };

    let new_group = super::leaguecord::data::Group::create_new(ctx.clone(), &data.ids)
        .await
        .unwrap();
    let code = new_group.invite_code.clone();
    data.groups.write().await.push(new_group);

    let mut x = data.invites.write().await;
    *x = super::leaguecord::build_invite_list(ctx.clone(), &data.ids)
        .await
        .unwrap();

    let _ = message.reply(ctx.http, format!("discord.gg/{code}")).await;
}

async fn cleanup(ctx: Context, message: &Message) {
    use crate::bot::command;
    let Some(_args) = command::parse(
        message,
        "cleanup",
        command::Case::Insensitive,
        command::Prefix::Yes,
    ) else {
        return;
    };

    let ctx_data_storage = ctx.data.clone();
    let ctx_data_storage_read = ctx_data_storage.read().await;
    let Some(data) = ctx_data_storage_read.get::<super::leaguecord::LeagueCordData>() else {
        error!("Could not get tracked invites from data");
        return;
    };

    let guild = ctx.http.get_guild(data.ids.guild).await.unwrap();

    // Channels
    {
        for (id, mut channel) in guild.channels(ctx.http.clone()).await.unwrap() {
            if !channel.name.starts_with("g") && channel.name.parse::<u64>().is_err()
                || channel.id == data.ids.graveyard_category
            {
                continue;
            }

            debug!("Deleting channel '{}'({id})", channel.name);

            if let Err(e) = channel.delete(ctx.http.clone()).await {
                error!("Failed due to: {e}");
                channel
                    .edit(
                        ctx.http.clone(),
                        EditChannel::new().category(data.ids.graveyard_category),
                    )
                    .await
                    .unwrap()
            }
        }
    }

    // Users
    {
        for member in guild.members(ctx.http.clone(), None, None).await.unwrap() {
            let mut delete = false;
            for role_id in member.roles.iter() {
                let role = ctx
                    .http
                    .get_guild_role(data.ids.guild, *role_id)
                    .await
                    .unwrap();
                if role.name.starts_with("group") {
                    delete = true;
                    break;
                }
            }
            if !delete {
                continue;
            }

            if let Err(e) = member.kick(ctx.http.clone()).await {
                error!("Failed to kick user '{}' due to: {e}", member.user.id)
            }
        }
    }

    // Roles
    {
        for (id, mut role) in guild.roles {
            if !role.name.starts_with("g") {
                continue;
            }

            debug!("Deleting role '{}'({id})", role.name);

            if let Err(e) = role.delete(ctx.http.clone()).await {
                error!("Failed due to: {e}");
            }
        }
    }
}
