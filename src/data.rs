use std::{collections::HashMap, sync::Arc};

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
