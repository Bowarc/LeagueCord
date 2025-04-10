pub struct Debug;

#[serenity::async_trait]
impl serenity::all::EventHandler for Debug {
    async fn ready(
        &self,
        ctx: serenity::all::Context,
        data_about_bot: serenity::model::prelude::Ready,
    ) {
        use serenity::all::{CacheHttp as _, CreateCommand};

        let guild = ctx
            .http
            .get_guild(data_about_bot.guilds.first().unwrap().id)
            .await
            .unwrap();

        guild
            .create_command(
                ctx.http(),
                CreateCommand::new("status").description("Check if the bot is awake"),
            )
            .await
            .unwrap();

        guild
            .create_command(
                ctx.http(),
                CreateCommand::new("devreport").description(
                    "Command to list different infos about the current activity of the bot",
                ),
            )
            .await
            .unwrap();
    }

    async fn message(&self, ctx: serenity::all::Context, message: serenity::all::Message) {
        use {crate::data::LeagueCordData, serenity::all::CacheHttp as _};

        super::module_command(&ctx, "Debug", message.clone()).await;

        let ctx_data_storage = ctx.data.clone();
        let ctx_data_storage_read = ctx_data_storage.read().await;
        let Some(data) = ctx_data_storage_read.get::<LeagueCordData>() else {
            error!("Could not get tracked invites from data");
            return;
        };

        if !super::has_admin_role(ctx.http(), &message.author, &data.ids).await {
            return;
        }

        create_group(&ctx, &message).await;
        reset(&ctx, &message).await;
    }

    async fn interaction_create(
        &self,
        ctx: serenity::all::Context,
        interaction: serenity::all::Interaction,
    ) {
        use {
            crate::data::LeagueCordData,
            serenity::all::{
                CacheHttp, CreateInteractionResponse, CreateInteractionResponseMessage, Interaction,
            },
        };

        let Interaction::Command(ci) = interaction else {
            return;
        };

        let ctx_data_storage = ctx.data.clone();
        let ctx_data_storage_read = ctx_data_storage.read().await;
        let Some(data) = ctx_data_storage_read.get::<LeagueCordData>() else {
            error!("Could not get tracked invites from data");
            return;
        };

        if !super::has_admin_role(ctx.http(), &ci.user, &data.ids).await {
            return;
        }

        match ci.data.name.as_str() {
            "ping" => {
                if let Err(e) = ci
                    .create_response(
                        ctx.http,
                        CreateInteractionResponse::Message(
                            CreateInteractionResponseMessage::new()
                                .content("I'm up !")
                                .ephemeral(true),
                        ),
                    )
                    .await
                {
                    error!(
                        "Failed to send a reponse to command {} due to: {e}",
                        ci.data.name
                    )
                }
            }
            "devreport" => devreport(ctx, ci).await,

            _ => (),
            // Might not be a debug command ! 
            // debug!("Unknown command from {} {}", ci.user.name, ci.data.name),
        }
    }
}

async fn create_group(ctx: &serenity::all::Context, message: &serenity::all::Message) {
    ///
    ///  SECURITY:
    ///    
    ///    Roles for the user of this command have been checked in the EventHandler::message method
    ///
    use {
        crate::{
            bot::command,
            data::{Group, LeagueCordData},
        },
        serenity::all::CacheHttp as _,
    };

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
    let Some(data) = ctx_data_storage_read.get::<LeagueCordData>() else {
        error!("Could not get tracked invites from data");
        return;
    };

    let new_group = Group::create_new(ctx.clone(), &data.ids).await.unwrap();
    let code = new_group.invite_code.clone();
    data.groups.write().await.push(new_group);

    data.invites
        .write()
        .await
        .update(ctx.http(), &data.ids)
        .await
        .unwrap();

    let _ = message
        .reply(ctx.http(), format!("discord.gg/{code}"))
        .await;
}

async fn reset(ctx: &serenity::all::Context, message: &serenity::all::Message) {
    ///
    ///  SECURITY:
    ///    
    ///    Roles for the user of this command have been checked in the EventHandler::message method
    ///
    use {
        crate::{bot::command, data::LeagueCordData},
        serenity::all::{CacheHttp as _, EditChannel},
    };

    let Some(_args) = command::parse(
        message,
        "reset",
        command::Case::Insensitive,
        command::Prefix::Yes,
    ) else {
        return;
    };

    let ctx_data_storage = ctx.data.clone();
    let ctx_data_storage_read = ctx_data_storage.read().await;

    let Some(data) = ctx_data_storage_read.get::<LeagueCordData>() else {
        error!("Could not get tracked invites from data");
        return;
    };

    // Groups
    {
        let mut invite_tracker = data.invites.write().await;
        for group in data.groups.read().await.iter() {
            group.cleanup_for_deletion(ctx.clone(), &data.ids).await;
            invite_tracker.rm(&group.invite_code);
        }
    }

    // Remove any other artefacts in case they were not registed in a group (ex. server restarted)

    let guild = ctx.http.get_guild(data.ids.guild).await.unwrap();

    // Channels
    {
        for (id, mut channel) in guild.channels(ctx.http()).await.unwrap() {
            if !channel.name.starts_with("g") && channel.name.parse::<u64>().is_err()
                || channel.id == data.ids.graveyard_category
            {
                continue;
            }

            debug!("Deleting channel '{}'({id})", channel.name);

            if let Err(e) = channel.delete(ctx.http()).await {
                error!("Failed due to: {e}");
                channel
                    .edit(
                        ctx.http(),
                        EditChannel::new().category(data.ids.graveyard_category),
                    )
                    .await
                    .unwrap()
            }
        }
    }

    // Users
    {
        for member in guild.members(ctx.http(), None, None).await.unwrap() {
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

            if let Err(e) = member.kick(ctx.http()).await {
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

            if let Err(e) = role.delete(ctx.http()).await {
                error!("Failed due to: {e}");
            }
        }
    }

    // Cleanup group storage
    {
        data.groups.write().await.clear();
    }
}

async fn devreport(ctx: serenity::all::Context, ci: serenity::all::CommandInteraction) {
    ///
    ///  SECURITY:
    ///    
    ///    Roles for the user of this command have been checked in the EventHandler::interaction_create method
    ///
    use {
        crate::data::LeagueCordData,
        // futures::StreamExt as _,
        serenity::all::{
            CreateEmbed, CreateInteractionResponse,
            CreateInteractionResponseMessage,
        },
    };

    let ctx_data_storage = ctx.data.clone();
    let ctx_data_storage_read = ctx_data_storage.read().await;
    let Some(data) = ctx_data_storage_read.get::<LeagueCordData>() else {
        error!("Could not get tracked invites from data");
        return;
    };

    if ci.channel_id != data.ids.bot_command_channel {
        if let Err(e) = ci
            .create_response(
                ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("You cannot use that command in this channel")
                        .ephemeral(true),
                ),
            )
            .await
        {
            error!("{e}");
        }
        return;
    }

    let mut embed = CreateEmbed::new()
        // .author(CreateEmbedAuthor::new("Leaguecord"))
        .color((36, 219, 144))
        .title("Dev report")
        .description("Display of basic leaguecord informations.\n");

    let groups = data.groups.read().await;
    let group_count = groups.len();
    let member_count: u32 = groups.iter().map(|g| g.users.len() as u32).sum();

    let losts = Vec::<serenity::all::Member>::new();

    // There is an issue with member_iter, the same member is repeated infinitely
    // while let Some(member_r) = data.ids.guild.members_iter(ctx.http()).boxed().next().await {
    //     debug!("Got member: {member_r:?}");
    //     let Ok(member) = member_r else {
    //         warn!("its an err");
    //         continue;
    //     };

    //     if !member.roles.contains(&data.ids.lost_role) {
    //         warn!("Is not lost");
    //                     continue;
    //     }
    //     debug!("is lost");

    //     losts.push(member);
    // }

    embed = embed.fields(vec![
        (
            "Groups",
            format!(
                "There are currently {group_count} group{group_s}, for a total of {member_count} members",
                group_s = if group_count > 1 { "s" } else { "" },
            ),
            false,
        ),
        (
            "Lost users",
            format!(
                "There are currently {} lost users in this server",
                losts.len()
            ),
            false,
        ),
        // TODO: more ?
    ]);

    if let Err(e) = ci
        .create_response(
            ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .embed(embed)
                    .ephemeral(true),
            ),
        )
        .await
    {
        error!(
            "Failed to send a reponse to command {} due to: {e}",
            ci.data.name
        )
    }
}
