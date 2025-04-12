#[derive(Default, Debug)]
pub struct GroupCreationSpamTracker(
    tokio::sync::RwLock<
        std::collections::HashMap<super::IpStruct, (std::time::Instant, super::GroupId)>,
    >,
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

    pub async fn has(&self, ip: super::IpStruct) -> Option<super::GroupId> {
        self.0.read().await.get(&ip).map(|(_, id)| *id)
    }

    pub async fn register(&self, ip: super::IpStruct, group_id: super::GroupId) {
        use std::time::Instant;

        if let Some(old) = self.0.write().await.insert(ip, (Instant::now(), group_id)) {
            // warn!("Registering a new group in the SpamTracker returned an old value: {old:?}");
            warn!(
                "SpamTracker registered a new group, but this ip already had a recent one: {old:?}"
            );
        }
    }

    pub async fn remove(&self, ip: super::IpStruct) {
        self.0.write().await.remove(&ip);
    }
}
