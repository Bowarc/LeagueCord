use std::time::UNIX_EPOCH;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct GroupData {
    id: u64,
    creation_time_s_since_epoch: u64,
    user_count: u32,
    invite_code: String,
}

impl GroupData {
    pub fn new(
        id: u64,
        creation_time: std::time::SystemTime,
        user_count: u32,
        invite_code: String,
    ) -> GroupData {
        Self {
            id,
            creation_time_s_since_epoch: creation_time
                .duration_since(UNIX_EPOCH)
                .map(|dur| dur.as_secs())
                .unwrap_or(0),
            user_count,
            invite_code,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn creation_time_s_since_epoch(&self) -> u64 {
        self.creation_time_s_since_epoch
    }

    pub fn user_count(&self) -> u32 {
        self.user_count
    }

    pub fn invite_code(&self) -> &String {
        &self.invite_code
    }
}
