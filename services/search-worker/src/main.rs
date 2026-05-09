mod models;
mod observability;

use std::time::Duration;
use tokio::time::sleep;
use meilisearch_sdk::client::Client;
use redis::{Client as RedisClient, Commands};
use crate::models::IndexingTask;
use std::env;
use std::error::Error;
use axum::{routing::get, Router};
use std::net::SocketAddr;
use metrics_exporter_prometheus::PrometheusBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    observability::init_observability("search-worker");

    let recorder_handle = PrometheusBuilder::new()
        .install_recorder()
        .expect("failed to install recorder");

    // Start Health/Metrics Server
    let health_router = Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/metrics", get(move || async move { recorder_handle.render() }));
    
    let addr = SocketAddr::from(([0, 0, 0, 0], 3002));
    tokio::spawn(async move {
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, health_router).await.unwrap();
    });

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
