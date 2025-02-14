use std::ops::Deref;

use futures::channel;
use serenity::all::EditChannel;

use {
    serenity::{
        all::{Context, EditMember, EditRole, EventHandler, GuildId, Message},
        prelude::TypeMapKey,
    },
    std::{collections::HashMap, sync::Arc},
    tokio::sync::RwLock,
};

pub struct Debug;

#[serenity::async_trait]
impl EventHandler for Debug {
    async fn ready(&self, ctx: Context, data_about_bot: serenity::model::prelude::Ready) {}

    async fn guild_member_addition(
        &self,
        ctx: Context,
        mut new_member: serenity::model::prelude::Member,
    ) {
    }

    async fn message(&self, ctx: Context, message: Message) {
        let ctx_data_storage = ctx.data.clone();
        let ctx_data_storage_read = ctx_data_storage.read().await;
        let Some(data) = ctx_data_storage_read.get::<super::leaguecord::LeagueCordData>() else {
            error!("Could not get tracked invites from data");
            return;
        };

        if !message
            .author
            .has_role(ctx.http.clone(), data.ids.guild, data.ids.admin_role).await.unwrap()
        {
            return;
        }
        create_group(ctx.clone(), &message).await;
        cleanup(ctx, &message).await
    }
}

async fn create_group(ctx: Context, message: &Message) {
    use crate::bot::command;
    let Some(args) = command::parse(
        &message,
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
    let Some(args) = command::parse(
        &message,
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
            if !channel.name.starts_with("group") && channel.name.parse::<u64>().is_err() {
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

    {
        for (id, mut role) in guild.roles {
            if !role.name.starts_with("group") {
                continue;
            }

            debug!("Deleting role '{}'({id})", role.name);

            if let Err(e) = role.delete(ctx.http.clone()).await {
                error!("Failed due to: {e}");
            }
        }
    }
}
