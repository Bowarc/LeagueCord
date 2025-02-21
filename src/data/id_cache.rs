use serenity::all::{ChannelId, GuildId, RoleId};

#[derive(Debug)]
pub struct IdCache {
    pub guild: GuildId,
    pub admin_role: RoleId,
    pub graveyard_category: ChannelId,
    pub bot_log_channel: ChannelId,
}

