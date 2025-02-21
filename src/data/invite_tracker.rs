use std::{collections::HashMap, time::Instant};

use serenity::all::CacheHttp;

use super::{IdCache, InviteCode, InviteUseCount};

#[derive(Debug)]
pub struct InviteTracker {
    storage: HashMap<InviteCode, InviteUseCount>,
    last_update: Instant,
}

impl InviteTracker {
    pub async fn new(http: impl CacheHttp, ids: &IdCache) -> Result<Self, String> {
        let mut it = Self {
            storage: HashMap::default(),
            last_update: Instant::now(),
        };

        it.update(http, ids).await?;

        Ok(it)
    }

    pub async fn update(&mut self, http: impl CacheHttp, ids: &IdCache) -> Result<(), String> {
        let Ok(invite_list) = http.http().get_guild_invites(ids.guild).await else {
            return Err(format!(
                "Could not get the invite list for guild: {:?}",
                ids.guild
            ));
        };

        self.storage = invite_list
            .into_iter()
            .map(|invite| (invite.code, invite.uses))
            .collect();

        self.last_update = Instant::now();

        Ok(())
    }
    #[inline]
    pub fn get(&self, code: &InviteCode) -> Option<&InviteUseCount>{
        self.storage.get(code)
    }
}
