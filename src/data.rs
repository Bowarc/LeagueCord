use std::{collections::HashMap, sync::Arc};

use serenity::prelude::TypeMapKey;
use tokio::sync::RwLock;

mod group;
mod id_cache;

#[allow(unused_imports)]
pub use group::{Group, GroupId};
pub use id_cache::IdCache;

pub type InviteCode = String;
pub type InviteUseCount = u64;

#[derive(Debug, Clone)]
pub struct LeagueCordData {
    pub ids: Arc<IdCache>,
    pub invites: Arc<RwLock<HashMap<InviteCode, InviteUseCount>>>,
    pub groups: Arc<RwLock<Vec<Group>>>,
}

impl TypeMapKey for LeagueCordData {
    type Value = Self;
}
