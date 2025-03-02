#[derive(Debug)]
pub struct IdCache {
    pub guild: serenity::all::GuildId,
    pub admin_role: serenity::all::RoleId,
    pub graveyard_category: serenity::all::ChannelId,
    pub bot_log_channel: serenity::all::ChannelId,
}
