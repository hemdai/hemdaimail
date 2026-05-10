use redis::{Client, Commands};
use std::error::Error;
use crate::models::tasks::SyncTask;
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

pub struct Queue {
    client: Client,
}

impl Queue {
    pub fn new(url: &str) -> Result<Self, Box<dyn Error>> {
        let client = Client::open(url)?;
        Ok(Queue { client })
    }

    pub fn push_sync_task(&self, task: SyncTask) -> Result<(), Box<dyn Error>> {
        let mut conn = self.client.get_connection()?;
        let json = serde_json::to_string(&task)?;
        let _: () = conn.lpush("mail_sync_tasks", json)?;
        Ok(())
    }

    pub fn pop_sync_task(&self) -> Result<Option<SyncTask>, Box<dyn Error>> {
        let mut conn = self.client.get_connection()?;
        let result: Option<String> = conn.rpop("mail_sync_tasks", None)?;
        
        match result {
            Some(json) => {
                let task: SyncTask = serde_json::from_str(&json)?;
                Ok(Some(task))
            }
            None => Ok(None),
        }
    }

    pub fn push_indexing_task(&self, task: IndexingTask) -> Result<(), Box<dyn Error>> {
        let mut conn = self.client.get_connection()?;
        let json = serde_json::to_string(&task)?;
        let _: () = conn.lpush("search_indexing_tasks", json)?;
        Ok(())
    }

    pub fn publish_event(&self, user_id: Uuid, event_type: &str, payload: serde_json::Value) -> Result<(), Box<dyn Error>> {
        let mut conn = self.client.get_connection()?;
        let channel = format!("user_events:{}", user_id);
        let event = serde_json::json!({
            "type": event_type,
            "payload": payload
        });
        let _: () = conn.publish(channel, event.to_string())?;
        Ok(())
    }
}
