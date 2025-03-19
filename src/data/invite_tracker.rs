#[derive(Debug)]
pub struct InviteTracker {
    storage: std::collections::HashMap<super::InviteCode, super::InviteUseCount>,
    last_update: std::time::Instant,
}

impl InviteTracker {
    pub async fn new(
        http: impl serenity::all::CacheHttp,
        ids: &super::IdCache,
    ) -> Result<Self, String> {
        use std::{collections::HashMap, time::Instant};

        let mut it = Self {
            storage: HashMap::default(),
            last_update: Instant::now(),
        };

        it.update(http, ids).await?;

        Ok(it)
    }

    pub async fn update(
        &mut self,
        http: impl serenity::all::CacheHttp,
        ids: &super::IdCache,
    ) -> Result<(), String> {
        use std::time::Instant;

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
    pub fn get(&self, code: &super::InviteCode) -> Option<&super::InviteUseCount> {
        self.storage.get(code)
    }

    pub fn set(&mut self, code: super::InviteCode, uc: super::InviteUseCount){
        use std::time::Instant;

        self.storage.insert(code, uc);
        self.last_update = Instant::now();
    }

    pub fn rm (&mut self, code: &super::InviteCode) {
        use std::time::Instant;

        self.storage.remove(code); // don't really care about errors
        self.last_update = Instant::now()
    }
}
