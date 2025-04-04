#[derive(Debug)]
pub struct IdCache {
    pub guild: serenity::all::GuildId,
    pub admin_role: serenity::all::RoleId,
    pub graveyard_category: serenity::all::ChannelId,
    pub bot_log_channel: serenity::all::ChannelId,
    pub bot_command_channel: serenity::all::ChannelId,
}

impl IdCache {
    pub async fn new(
        ctx: serenity::all::Context,
        guild_id: serenity::all::GuildId,
    ) -> Result<Self, String> {
        use serenity::all::{CacheHttp, ChannelType};

        let leaguecord_management_category = find_or_create_channel(
            ctx.clone(),
            guild_id,
            "leaguecord management",
            None,
            ChannelType::Category,
        )
        .await?;

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
