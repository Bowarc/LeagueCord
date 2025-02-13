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
}

#[derive(Debug)]
pub struct Group {
    creation_time: Instant,
    invite_code: String,
    text_channel: ChannelId,
    voice_channel: ChannelId,

    users: Vec<UserId>,
}

impl Group {
    pub async fn create_new(ctx: Context, ids: &IdCache) -> Result<Self, String> {
        let id = random::get(0, u64::MAX);

        let guild = ctx
            .http
            .clone()
            .get_guild(ids.guild)
            .await
            .map_err(|e| e.to_string())?;

        let text_channel = guild
            .create_channel(
                ctx.http.clone(),
                CreateChannel::new(id.to_string()).kind(ChannelType::Text),
            )
            .await
            .map_err(|e| e.to_string())?;

        let voice_channel = guild
            .create_channel(
                ctx.http.clone(),
                CreateChannel::new(id.to_string()).kind(ChannelType::Voice),
            )
            .await
            .map_err(|e| e.to_string())?;

        let invite = text_channel
            .create_invite(
                ctx,
                CreateInvite::new()
                    .max_age(900)
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

            users: Vec::new(),
        })
    }
}
