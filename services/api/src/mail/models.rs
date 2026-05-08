use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "mailbox_type", rename_all = "lowercase")]
pub enum MailboxType {
    Inbox,
    Sent,
    Drafts,
    Trash,
    Spam,
    Archive,
    Custom,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Mailbox {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub r#type: MailboxType,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Message {
    pub id: Uuid,
    pub user_id: Uuid,
    pub mailbox_id: Uuid,
    pub thread_id: Option<Uuid>,
    pub sender: String,
    pub recipients: Vec<String>,
    pub subject: Option<String>,
    pub snippet: Option<String>,
    pub is_read: bool,
    pub is_starred: bool,
    pub is_archived: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct SendEmailRequest {
    pub to: Vec<String>,
    pub subject: String,
    pub body: String,
}
