mod db;
mod imap;
mod models;

use std::time::Duration;
use tokio::time::sleep;
use sqlx::PgPool;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    tracing::info!("Mail worker starting...");

    let pool = db::connect_db().await;

    // TODO: Connect to Redis for job queue
    
    // Example: Triggering a sync (This would normally come from a queue)
    run_worker_loop(pool).await?;

    Ok(())
}

async fn run_worker_loop(_pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        tracing::debug!("Worker heartbeat...");
        // In reality, we'd pop from Redis here
        sleep(Duration::from_secs(30)).await;
    }
}
