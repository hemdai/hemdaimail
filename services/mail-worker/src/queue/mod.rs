use redis::{Client, Commands, Connection};
use std::error::Error;
use crate::models::tasks::SyncTask;

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
}
