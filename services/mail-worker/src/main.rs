mod db;

use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    tracing::info!("Mail worker starting...");

    let _pool = db::connect_db().await;

    // TODO: Connect to Redis for job queue
    // TODO: Implement IMAP sync loop

    loop {
        tracing::debug!("Worker heartbeat...");
        sleep(Duration::from_secs(30)).await;
    }
}
