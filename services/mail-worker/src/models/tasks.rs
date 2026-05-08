use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub enum SyncType {
    Full,
    Incremental,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncTask {
    pub user_id: Uuid,
    pub mailbox_name: String,
    pub sync_type: SyncType,
}
