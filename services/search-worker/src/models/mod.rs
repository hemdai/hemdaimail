use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexingTask {
    pub message_id: Uuid,
    pub user_id: Uuid,
    pub subject: Option<String>,
    pub sender: String,
    pub body_text: Option<String>,
    pub created_at: String,
    pub correlation_id: Option<String>,
}
