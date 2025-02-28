use std::{
    collections::HashMap,
    net::IpAddr,
    sync::Arc,
    time::{Duration, Instant},
};

use serenity::prelude::TypeMapKey;
use tokio::sync::RwLock;

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
    pub ids: Arc<IdCache>,
    pub invites: Arc<RwLock<InviteTracker>>,
    pub groups: Arc<RwLock<Vec<Group>>>,
}

impl TypeMapKey for LeagueCordData {
    type Value = Self;
}

#[derive(Default, Debug)]
pub struct GroupCreationSpamTracker(RwLock<HashMap<IpAddr, (Instant, GroupId)>>);

const TRACKER_DURATION: Duration = Duration::from_secs(60 * 5); // An ip can create a group every 5 minutes

impl GroupCreationSpamTracker {
    pub async fn update(&self) {
        let now = Instant::now(); // Don't re-compute it each time
        self.0
            .write()
            .await
            .retain(|_ip, (instant, _id)| now - *instant < TRACKER_DURATION);
    }

    pub async fn has(&self, ip: IpAddr) -> Option<GroupId> {
        self.0.read().await.get(&ip).map(|(_, id)| *id)
    }

    pub async fn register(&self, ip: IpAddr, group_id: GroupId) {
        if let Some(old) = self.0.write().await.insert(ip, (Instant::now(), group_id)) {
            warn!("Registering a new group in the SpamTracker returned an old value: {old:?}");
        }
    }
}
