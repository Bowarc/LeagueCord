mod group;
mod id_cache;
mod invite_tracker;

#[allow(unused_imports)]
pub use group::{Group, GroupId};
pub use id_cache::IdCache;
pub use invite_tracker::InviteTracker;

pub type InviteCode = String;
pub type InviteUseCount = u64;

#[derive(Debug, Clone)]
pub struct LeagueCordData {
    pub ids: std::sync::Arc<IdCache>,
    pub invites: std::sync::Arc<tokio::sync::RwLock<InviteTracker>>,
    pub groups: std::sync::Arc<tokio::sync::RwLock<Vec<Group>>>,
}

impl serenity::prelude::TypeMapKey for LeagueCordData {
    type Value = Self;
}

#[derive(Default, Debug)]
pub struct GroupCreationSpamTracker(
    tokio::sync::RwLock<std::collections::HashMap<std::net::IpAddr, (std::time::Instant, GroupId)>>,
);

const TRACKER_DURATION: std::time::Duration = std::time::Duration::from_secs(60 * 5); // An ip can create a group every 5 minutes

impl GroupCreationSpamTracker {
    pub async fn update(&self) {
        use std::time::Instant;

        let now = Instant::now(); // Don't re-compute it each time
        self.0
            .write()
            .await
            .retain(|_ip, (instant, _id)| now - *instant < TRACKER_DURATION);
    }

    // pub async fn has(&self, ip: std::net::IpAddr) -> Option<GroupId> {
    //     self.0.read().await.get(&ip).map(|(_, id)| *id)
    // }

    pub async fn register(&self, ip: std::net::IpAddr, group_id: GroupId) {
        use std::time::Instant;

        if let Some(old) = self.0.write().await.insert(ip, (Instant::now(), group_id)) {
            // warn!("Registering a new group in the SpamTracker returned an old value: {old:?}");
            warn!(
                "SpamTracker registered a new group, but this ip already had a recent one: {old:?}"
            );
        }
    }

    // pub async fn remove(&self, ip: std::net::IpAddr) {
    //     self.0.write().await.remove(&ip);
    // }
}
