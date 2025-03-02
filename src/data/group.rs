pub type GroupId = u64;

#[derive(Debug)]
pub struct Group {
    pub id: GroupId,

    pub creation_time: std::time::Instant,
    pub invite_code: String,
    pub text_channel: serenity::all::ChannelId,
    pub voice_channel: serenity::all::ChannelId,
    pub category: serenity::all::ChannelId,
    pub role: serenity::all::RoleId,

    pub users: Vec<serenity::all::UserId>,
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

    pub async fn create_new(
        http: impl serenity::all::CacheHttp,
        ids: &super::IdCache,
    ) -> Result<Self, String> {
        use {
            serenity::all::{
                ChannelType, CreateChannel, CreateInvite, EditRole, PermissionOverwrite,
                PermissionOverwriteType, Permissions,
            },
            std::time::Instant,
        };

        let http = http.http();
        let group_id = random::get(0, u64::MAX);

        let name = Self::name_for_id(group_id);

        let guild = http.get_guild(ids.guild).await.map_err(|e| e.to_string())?;

        let (category, role) = match futures::join!(
            guild.create_channel(http, CreateChannel::new(&name).kind(ChannelType::Category),),
            guild.create_role(http, EditRole::new().name(&name))
        ) {
            (Ok(c), Ok(r)) => (c, r),
            (Ok(c), Err(e)) => {
                if let Err(e) = c.delete(http).await {
                    error!("Failed to cleanup role due to {e}");
                }
                return Err(e.to_string());
            }
            (Err(e), Ok(mut r)) => {
                if let Err(e) = r.delete(http).await {
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
            guild.create_channel(http, channel_base.clone().kind(ChannelType::Text),),
            guild.create_channel(http, channel_base.kind(ChannelType::Voice))
        ) {
            (Ok(text), Ok(voice)) => (text, voice),
            (Ok(c), Err(e)) | (Err(e), Ok(c)) => {
                if let Err(e) = c.delete(http).await {
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
                http,
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

    pub async fn cleanup_for_deletion(&self, ctx: serenity::all::Context, ids: &super::IdCache) {
        use tokio::task::JoinSet;

        // The invitation is automatically deleted when removing the associated channel

        // kick users
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

        // remove channels
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

        // remove role
        if let Err(e) = ids.guild.delete_role(ctx.http, self.role).await {
            error!("Failed to delete role for group {} due to {e}", self.id)
        }
    }
}

impl Group {
    pub fn to_data(&self) -> shared::GroupData {
        shared::GroupData::new(self.id, self.users.len() as u32, self.invite_code.clone())
    }
}
