#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct GroupData {
    id: u64,
    user_count: u32,
    invite_code: String,
}

impl GroupData {
    pub fn new(id: u64, user_count: u32, invite_code: String) -> GroupData {
        Self {
            id,
            user_count,
            invite_code,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn user_count(&self) -> u32 {
        self.user_count
    }

    pub fn invite_code(&self) -> &String {
        &self.invite_code
    }
}
