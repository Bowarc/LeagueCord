mod group;
mod id_cache;
mod invite_tracker;
mod gc_spam_tracker;

#[allow(unused_imports)]
pub use group::{Group, GroupId};
pub use id_cache::IdCache;
pub use invite_tracker::InviteTracker;
pub use gc_spam_tracker::GroupCreationSpamTracker;

pub type InviteCode = String;
pub type InviteUseCount = u64;
pub type IpStruct = rocket_client_addr::ClientAddr;

#[derive(Debug, Clone)]
pub struct LeagueCordData {
    pub ids: std::sync::Arc<IdCache>,
    pub invites: std::sync::Arc<tokio::sync::RwLock<InviteTracker>>,
    pub groups: std::sync::Arc<tokio::sync::RwLock<Vec<Group>>>,
}

impl serenity::prelude::TypeMapKey for LeagueCordData {
    type Value = Self;
}

