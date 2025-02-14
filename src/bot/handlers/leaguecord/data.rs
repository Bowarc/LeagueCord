use serenity::all::{EditRole, PermissionOverwrite, PermissionOverwriteType, Permissions, RoleId};

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

#[derive(Debug)]
pub struct Group {
    pub creation_time: Instant,
    pub invite_code: String,
    pub text_channel: ChannelId,
    pub voice_channel: ChannelId,
    pub role: RoleId,

    pub users: Vec<UserId>,
}

impl Group {
    pub async fn create_new(ctx: Context, ids: &IdCache) -> Result<Self, String> {
        let id = random::get(0, u64::MAX);

        let name = format!("group-{id}");

        let guild = ctx
            .http
            .clone()
            .get_guild(ids.guild)
            .await
            .map_err(|e| e.to_string())?;

        let category = guild
            .create_channel(
                ctx.http.clone(),
                CreateChannel::new(id.to_string()).kind(ChannelType::Category),
            )
            .await
            .map_err(|e| e.to_string())?;

        let role = guild
            .create_role(ctx.http.clone(), EditRole::new().name(&name))
            .await
            .map_err(|e| e.to_string())?;

        let channel_base = CreateChannel::new(name).category(category.id).permissions({
            let group_user_permissions = Permissions::VIEW_CHANNEL
                | Permissions::CONNECT
                | Permissions::SEND_MESSAGES
                | Permissions::MANAGE_MESSAGES
                | Permissions::READ_MESSAGE_HISTORY
                | Permissions::ADD_REACTIONS
                | Permissions::SPEAK
                | Permissions::USE_VAD;

            vec![
                // ADMIN
                PermissionOverwrite {
                    allow: group_user_permissions
                        | Permissions::MANAGE_CHANNELS
                        | Permissions::PRIORITY_SPEAKER,
                    deny: Permissions::all(),
                    kind: PermissionOverwriteType::Role(ids.admin_role),
                },
                // GROUP USER
                PermissionOverwrite {
                    allow: group_user_permissions,
                    deny: Permissions::all(),
                    kind: PermissionOverwriteType::Role(role.id),
                },
            ]
        });

        let text_channel = guild
            .create_channel(
                ctx.http.clone(),
                channel_base.clone().kind(ChannelType::Text),
            )
            .await
            .map_err(|e| e.to_string())?;

        let voice_channel = guild
            .create_channel(ctx.http.clone(), channel_base.kind(ChannelType::Voice))
            .await
            .map_err(|e| e.to_string())?;

        let invite = text_channel
            .create_invite(
                ctx,
                CreateInvite::new()
                    .max_age(900) // 15 mins
                    .unique(true)
                    .max_uses(0)
                    .audit_log_reason("group channel invite"),
            )
            .await
            .map_err(|e| e.to_string())?;

        debug!("Created group {id}");

        Ok(Self {
            creation_time: Instant::now(),
            invite_code: invite.code,
            text_channel: text_channel.id,
            voice_channel: voice_channel.id,
            role: role.id,

            users: Vec::new(),
        })
    }
}
