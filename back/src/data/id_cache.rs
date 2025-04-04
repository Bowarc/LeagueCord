#[derive(Debug)]
pub struct IdCache {
    pub guild: serenity::all::GuildId,
    pub admin_role: serenity::all::RoleId,
    pub lost_role: serenity::all::RoleId,
    pub graveyard_category: serenity::all::ChannelId,
    pub bot_log_channel: serenity::all::ChannelId,
    pub bot_command_channel: serenity::all::ChannelId,
    pub lost_channel: serenity::all::ChannelId,
}

impl IdCache {
    pub async fn new(
        ctx: serenity::all::Context,
        guild_id: serenity::all::GuildId,
    ) -> Result<Self, String> {
        use serenity::all::{
            CacheHttp, ChannelType, CreateEmbed, CreateMessage, GetMessages, PermissionOverwrite,
            PermissionOverwriteType, Permissions,
        };

        let leaguecord_management_category = find_or_create_channel(
            ctx.clone(),
            guild_id,
            "leaguecord management",
            None,
            ChannelType::Category,
        )
        .await?;

        let lost_role = *guild_id
            .roles(ctx.http())
            .await
            .map_err(|e| e.to_string())?
            .iter()
            .find(|(_id, role)| role.name == "lost")
            .ok_or(String::from("Could not find admin role"))?
            .0;

        let community_category = find_or_create_channel(
            ctx.clone(),
            guild_id,
            "Community",
            None,
            ChannelType::Category,
        )
        .await?;

        community_category
            .create_permission(
                ctx.http(),
                PermissionOverwrite {
                    allow: Permissions::READ_MESSAGE_HISTORY | Permissions::VIEW_CHANNEL,
                    deny: Permissions::all(),
                    kind: PermissionOverwriteType::Role(lost_role),
                },
            )
            .await
            .map_err(|e| e.to_string())?;

        let lost_channel = find_or_create_channel(
            ctx.clone(),
            guild_id,
            "lost-users",
            Some(community_category),
            ChannelType::Text,
        )
        .await?;

        if lost_channel
            .messages(ctx.http(), GetMessages::new().limit(1))
            .await
            .map_err(|e| e.to_string())?
            .is_empty()
        {
            lost_channel
                .send_message(
                    ctx.http(),
                    CreateMessage::new().add_embed(CreateEmbed::new().field("", "Hi,\n\nLooks like our system could not find what group you tried to join.\n\nTo fix this, please try leaving the server and joining again using the same link.\n\nI'm working on a fix.", false).color((36, 219, 144))),
                )
                .await
                .map_err(|e| e.to_string())?;
        }

        Ok(Self {
            guild: guild_id,
            admin_role: *guild_id
                .roles(ctx.http())
                .await
                .map_err(|e| e.to_string())?
                .iter()
                .find(|(_id, role)| role.name == "-")
                .ok_or(String::from("Could not find admin role"))?
                .0,
            lost_role,
            graveyard_category: find_or_create_channel(
                ctx.clone(),
                guild_id,
                "graveyard",
                None,
                ChannelType::Category,
            )
            .await?,
            bot_log_channel: find_or_create_channel(
                ctx.clone(),
                guild_id,
                "bot_logs",
                Some(leaguecord_management_category),
                ChannelType::Text,
            )
            .await?,
            bot_command_channel: find_or_create_channel(
                ctx.clone(),
                guild_id,
                "commands",
                Some(leaguecord_management_category),
                ChannelType::Text,
            )
            .await?,
            lost_channel,
        })
    }
}

async fn find_or_create_channel(
    ctx: serenity::all::Context,
    guild_id: serenity::all::GuildId,
    name: &str,
    category_id_opt: Option<serenity::all::ChannelId>,
    kind: serenity::all::ChannelType,
) -> Result<serenity::all::ChannelId, String> {
    use serenity::all::{CacheHttp, CreateChannel};

    if let Some(channel) = ctx
        .http()
        .get_channels(guild_id)
        .await
        .map_err(|e| e.to_string())?
        .iter()
        .find(|channel| channel.name == name)
    {
        if channel.parent_id == category_id_opt {
            return Ok(channel.id);
        }
    }

    let mut create_channel = CreateChannel::new(name).kind(kind);

    if let Some(category_id) = category_id_opt {
        create_channel = create_channel.category(category_id);
    }

    Ok(guild_id
        .create_channel(ctx.http(), create_channel)
        .await
        .map_err(|e| e.to_string())?
        .id)
}
