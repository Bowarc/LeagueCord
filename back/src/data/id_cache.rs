#[derive(Debug)]
pub struct IdCache {
    pub guild: serenity::all::GuildId,
    pub admin_role: serenity::all::RoleId,
    pub lost_role: serenity::all::RoleId,
    pub graveyard_category: serenity::all::ChannelId,
    pub bot_log_channel: serenity::all::ChannelId,
    pub bot_command_channel: serenity::all::ChannelId,
    // pub lost_channel: serenity::all::ChannelId,
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

        let mut guild_channels = ctx
            .http
            .get_channels(guild_id)
            .await
            .map_err(|e| e.to_string())?;

        let leaguecord_management_category = find_or_create_channel(
            ctx.clone(),
            guild_id,
            &mut guild_channels,
            "leaguecord management",
            None,
            ChannelType::Category,
        )
        .await?;

        let lost_role = find_or_create_role(
            ctx.clone(),
            guild_id,
            "lost",
            (0, 0, 0),
            Permissions::empty(),
        )
        .await?;

        let community_category = find_or_create_channel(
            ctx.clone(),
            guild_id,
            &mut guild_channels,
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
            &mut guild_channels,
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
                    CreateMessage::new().add_embed(CreateEmbed::new().field("", "Hi,\n\nLooks like our system could not find what group you tried to join.\n\nPlease try leaving the server and joining again using the same link.\n\nI'm working on a fix.", false).color((36, 219, 144))),
                )
                .await
                .map_err(|e| e.to_string())?;
        }

        ensure_role_permissions(
            ctx.http(),
            guild_id,
            guild_id.everyone_role(),
            Permissions::empty(), //.union(Permissions::USE_APPLICATION_COMMANDS)
        )
        .await?;

        Ok(Self {
            guild: guild_id,
            admin_role: find_or_create_role(
                ctx.clone(),
                guild_id,
                "-",
                (36, 219, 144),
                Permissions::empty().union(Permissions::ADMINISTRATOR),
            )
            .await?,
            lost_role,
            graveyard_category: find_or_create_channel(
                ctx.clone(),
                guild_id,
                &mut guild_channels,
                "graveyard",
                None,
                ChannelType::Category,
            )
            .await?,
            bot_log_channel: find_or_create_channel(
                ctx.clone(),
                guild_id,
                &mut guild_channels,
                "bot_logs",
                Some(leaguecord_management_category),
                ChannelType::Text,
            )
            .await?,
            bot_command_channel: find_or_create_channel(
                ctx.clone(),
                guild_id,
                &mut guild_channels,
                "commands",
                Some(leaguecord_management_category),
                ChannelType::Text,
            )
            .await?,
            // lost_channel,
        })
    }
}

async fn find_or_create_channel(
    ctx: serenity::all::Context,
    guild_id: serenity::all::GuildId,
    guild_channels: &mut Vec<serenity::all::GuildChannel>,
    name: &str,
    category_id_opt: Option<serenity::all::ChannelId>,
    kind: serenity::all::ChannelType,
) -> Result<serenity::all::ChannelId, String> {
    use serenity::all::{CacheHttp, CreateChannel};

    let find = || {
        guild_channels
            .iter()
            .find(|channel| channel.name == name && channel.parent_id == category_id_opt)
    };

    if let Some(channel) = find() {
        return Ok(channel.id);
    }

    let mut create_channel = CreateChannel::new(name).kind(kind);

    if let Some(category_id) = category_id_opt {
        create_channel = create_channel.category(category_id);
    }

    let channel = guild_id
        .create_channel(ctx.http(), create_channel)
        .await
        .map_err(|e| e.to_string())?;

    let c_id = channel.id;

    guild_channels.push(channel);

    Ok(c_id)
}

async fn find_or_create_role(
    ctx: serenity::all::Context,
    guild_id: serenity::all::GuildId,
    name: &str,
    color: impl Into<serenity::all::Color>,
    permissions: serenity::all::Permissions,
) -> Result<serenity::all::RoleId, String> {
    use serenity::all::{CacheHttp, EditRole};

    let mut roles_res = guild_id
        .roles(ctx.http())
        .await
        .map_err(|e| e.to_string())?;

    let Some((id, role)) = roles_res.iter_mut().find(|(_id, role)| role.name == name) else {
        warn!("Could not find '{name}' role, creating it . .");

        match guild_id
            .create_role(
                ctx.http(),
                EditRole::new()
                    .colour(color)
                    .name(name)
                    .permissions(permissions),
            )
            .await
        {
            Ok(role) => return Ok(role.id),
            Err(e) => return Err(format!("Could not create role '{name}' due to: {e}")),
        }
    };

    let color = color.into();

    if role.colour != color {
        if let Err(e) = role.edit(ctx.http(), EditRole::new().colour(color)).await {
            return Err(format!(
                "Failed to edit the color of role '{name}' due to: {e}"
            ));
        }
    }

    ensure_role_permissions(ctx.http(), guild_id, *id, permissions).await?;

    Ok(*id)
}

async fn ensure_role_permissions(
    http: impl serenity::all::CacheHttp,
    guild_id: serenity::all::GuildId,
    role_id: serenity::all::RoleId,
    expected: serenity::all::Permissions,
) -> Result<(), String> {
    use serenity::all::EditRole;

    let mut role = http
        .http()
        .get_guild_role(guild_id, role_id)
        .await
        .map_err(|e| e.to_string())?;

    if role.permissions == expected {
        return Ok(());
    }

    if let Err(e) = role
        .edit(http.http(), EditRole::new().permissions(expected))
        .await
    {
        return Err(format!(
            "Failed to edit permissions of role '{}' due to: {e}, has {:?} instead of {:?}",
            role.name, role.permissions, expected
        ));
    }

    Ok(())
}
