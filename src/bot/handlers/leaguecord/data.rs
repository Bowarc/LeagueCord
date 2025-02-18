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

pub type GroupId = u64;

#[derive(Debug)]
pub struct Group {
    pub id: GroupId,

    pub creation_time: Instant,
    pub invite_code: String,
    pub text_channel: ChannelId,
    pub voice_channel: ChannelId,
    pub category: ChannelId,
    pub role: RoleId,

    pub users: Vec<UserId>,
}

impl Group {
    pub fn name_for_id(id: GroupId) -> String {
        format!("g{id}")
    }

    pub fn id_for_name(name: &str) -> Option<GroupId> {
        if !name.starts_with('g') {
            return None;
        }
        name[1..].parse::<GroupId>().ok()
    }

    pub async fn create_new(ctx: Context, ids: &IdCache) -> Result<Self, String> {
        let group_id = random::get(0, u64::MAX);

        let name = Self::name_for_id(group_id);

        let guild = ctx
            .http
            .clone()
            .get_guild(ids.guild)
            .await
            .map_err(|e| e.to_string())?;

        let (category, role) = match futures::join!(
            guild.create_channel(
                ctx.http.clone(),
                CreateChannel::new(&name).kind(ChannelType::Category),
            ),
            guild.create_role(ctx.http.clone(), EditRole::new().name(&name))
        ) {
            (Ok(c), Ok(r)) => (c, r),
            (Ok(c), Err(e)) => {
                if let Err(e) = c.delete(ctx.http.clone()).await {
                    error!("Failed to cleanup role due to {e}");
                }
                return Err(e.to_string());
            }
            (Err(e), Ok(mut r)) => {
                if let Err(e) = r.delete(ctx.http.clone()).await {
                    error!("Failed to cleanup role due to {e}");
                }
                return Err(e.to_string());
            }
            (Err(e1), Err(e2)) => {
                return Err(format!("{e1} AND {e2}"));
            }
        };

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

        let (text_channel, voice_channel) = match futures::join!(
            guild.create_channel(
                ctx.http.clone(),
                channel_base.clone().kind(ChannelType::Text),
            ),
            guild.create_channel(ctx.http.clone(), channel_base.kind(ChannelType::Voice))
        ) {
            (Ok(text), Ok(voice)) => (text, voice),
            (Ok(c), Err(e)) | (Err(e), Ok(c)) => {
                if let Err(e) = c.delete(ctx.http.clone()).await {
                    error!("Failed to cleanup channel {c} due to: {e}");
                }
                return Err(e.to_string());
            }
            (Err(e1), Err(e2)) => {
                return Err(format!("{e1} AND {e2}"));
            }
        };

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

        debug!("Created group {group_id}");

        Ok(Self {
            id: group_id,
            creation_time: Instant::now(),
            invite_code: invite.code,
            text_channel: text_channel.id,
            voice_channel: voice_channel.id,
            category: category.id,
            role: role.id,

            users: Vec::new(),
        })
    }

    pub async fn cleanup_for_deletion(&self, ctx: Context, ids: &IdCache) {
        // The invitation is automatically deleted when removing the associated channel

        for id in self.users.iter() {
            if let Err(e) = ctx
                .http
                .clone()
                .kick_member(ids.guild, *id, Some("Group cleanup"))
                .await
            {
                error!("Failed to kick user {id} due to {e}");
            }
        }

        let mut set = JoinSet::new();

        set.spawn(self.voice_channel.delete(ctx.http.clone()));
        set.spawn(self.text_channel.delete(ctx.http.clone()));
        set.spawn(self.category.delete(ctx.http.clone()));

        while let Some(res) = set.join_next().await {
            match res {
                Ok(Err(e)) => {
                    error!("Failed to delete a channel of group {} due to {e}", self.id)
                }
                Err(e) => {
                    error!("Failed to join group channel deletion future due to: {e}")
                }
                _ => (),
            }
        }

        if let Err(e) = ids.guild.delete_role(ctx.http, self.role).await {
            error!("Failed to delete role for group {} due to {e}", self.id)
        }
    }
}
