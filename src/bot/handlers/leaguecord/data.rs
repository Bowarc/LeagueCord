use serenity::all::{EditRole, PermissionOverwrite, PermissionOverwriteType, Permissions, RoleId};
use tokio::task::JoinSet;

use {
    serenity::all::{
        ChannelId, ChannelType, Context, CreateChannel, CreateInvite, GuildId, UserId,
    },
    std::time::Instant,
};

pub type InviteCode = String;
pub type InviteUseCount = u64;

#[derive(Debug)]
pub struct IdCache {
    pub guild: GuildId,
    pub admin_role: RoleId,
    pub graveyard_category: ChannelId,
    pub bot_log_channel: ChannelId,
}

