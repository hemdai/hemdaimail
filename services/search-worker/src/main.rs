mod models;

use std::time::Duration;
use tokio::time::sleep;
use meilisearch_sdk::client::Client;
use redis::{Client as RedisClient, Commands};
use crate::models::IndexingTask;
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    tracing::info!("Search worker starting...");

    let meili_url = env::var("MEILI_URL").expect("MEILI_URL must be set");
    let meili_key = env::var("MEILI_MASTER_KEY").expect("MEILI_MASTER_KEY must be set");
    let meili_client = Client::new(&meili_url, Some(&meili_key));

    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    let redis_client = RedisClient::open(redis_url)?;

    run_worker_loop(meili_client, redis_client).await?;

    Ok(())
}

async fn run_worker_loop(meili: Client, redis: RedisClient) -> Result<(), Box<dyn Error>> {
    let index = meili.index("messages");

    loop {
        let mut conn = redis.get_connection()?;
        let result: Option<String> = conn.rpop("search_indexing_tasks", None)?;

        match result {
            Some(json) => {
                let task: IndexingTask = serde_json::from_str(&json)?;
                tracing::info!("Indexing message {}", task.message_id);

                index.add_documents(&[task], Some("message_id")).await?;
            }
            None => {
                sleep(Duration::from_secs(5)).await;
            }
        }
    }
}
