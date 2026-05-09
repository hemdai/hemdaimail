mod db;
mod db_ops;
mod imap;
mod models;
mod queue;
mod storage;
mod observability;

use std::time::Duration;
use tokio::time::sleep;
use sqlx::PgPool;
use crate::imap::{ImapClient, fetch_new_messages, sync_mailboxes};
use crate::imap::processor::process_raw_message;
use crate::queue::{Queue, IndexingTask};
use crate::storage::Storage;
use uuid::Uuid;
use std::env;
use std::error::Error;
use chrono::Utc;
use axum::{routing::get, Router};
use std::net::SocketAddr;
use metrics_exporter_prometheus::PrometheusBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    observability::init_observability("mail-worker");

    let recorder_handle = PrometheusBuilder::new()
        .install_recorder()
        .expect("failed to install recorder");

    // Start Health/Metrics Server
    let health_router = Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/metrics", get(move || async move { recorder_handle.render() }));
    
    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    tokio::spawn(async move {
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, health_router).await.unwrap();
    });

    tracing::info!("Mail worker starting...");

    let pool = db::connect_db().await;
    
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    let queue = Queue::new(&redis_url)?;

    let storage = Storage::new().await;

    run_worker_loop(pool, queue, storage).await?;

    Ok(())
}

async fn run_worker_loop(pool: PgPool, queue: Queue, storage: Storage) -> Result<(), Box<dyn Error>> {
    loop {
        match queue.pop_sync_task() {
            Ok(Some(task)) => {
                tracing::info!("Processing sync task for user {}", task.user_id);
                if let Err(e) = sync_user_mail(&pool, &storage, &queue, task.user_id).await {
                    tracing::error!("Failed to sync user {}: {}", task.user_id, e);
                }
            }
            Ok(None) => {
                sleep(Duration::from_secs(5)).await;
            }
            Err(e) => {
                tracing::error!("Queue error: {}", e);
                sleep(Duration::from_secs(10)).await;
            }
        }
    }
}

async fn sync_user_mail(pool: &PgPool, storage: &Storage, queue: &Queue, user_id: Uuid) -> Result<(), Box<dyn Error>> {
    tracing::info!("Syncing mail for user {}", user_id);

    let creds = sqlx::query!(
        "SELECT host, port, username, password_encrypted FROM user_imap_credentials WHERE user_id = $1",
        user_id
    )
    .fetch_optional(pool)
    .await?;

    let creds = match creds {
        Some(c) => c,
        None => {
            tracing::warn!("No IMAP credentials for user {}", user_id);
            return Ok(());
        }
    };

    let client = ImapClient {
        host: creds.host,
        port: creds.port as u16,
        username: creds.username,
        password_encrypted: creds.password_encrypted,
    };

    let mut session = client.connect().await?;
    let remote_mailboxes = sync_mailboxes(&mut session).await?;
    
    for mb_name in remote_mailboxes {
        let mailbox = sqlx::query!(
            "INSERT INTO mailboxes (user_id, name) VALUES ($1, $2) ON CONFLICT (user_id, name) DO UPDATE SET name = EXCLUDED.name RETURNING id, last_uid_next",
            user_id,
            mb_name
        )
        .fetch_one(pool)
        .await?;

        let last_uid = mailbox.last_uid_next.map(|u| u as u32);
        let new_messages = fetch_new_messages(&mut session, &mb_name, last_uid).await?;

        for (uid, raw_mime) in new_messages {
            let processed = process_raw_message(&raw_mime)?;
            
            let message_id_key = processed.message_id.clone().unwrap_or_else(|| Uuid::new_v4().to_string());
            let s3_key = storage.upload_raw_mime(&message_id_key, raw_mime).await?;

            let message_id = db_ops::save_message(pool, user_id, mailbox.id, uid, processed.clone(), s3_key).await?;
            
            for attachment in processed.attachments {
                let attachment_key = storage.upload_attachment(&message_id_key, &attachment.filename, attachment.content.clone()).await?;
                db_ops::save_attachment(pool, message_id, &attachment.filename, &attachment.content_type, attachment.size, &attachment_key).await?;
            }

            // Push to Search Indexing Queue
            queue.push_indexing_task(IndexingTask {
                message_id,
                user_id,
                subject: processed.subject.clone(),
                sender: processed.from.clone().unwrap_or_default(),
                body_text: processed.body_text.clone(),
                created_at: Utc::now().to_rfc3339(),
            })?;

            // Publish Real-time Event
            queue.publish_event(user_id, "NEW_MESSAGE", serde_json::json!({
                "id": message_id,
                "subject": processed.subject,
                "sender": processed.from,
                "snippet": processed.body_text.as_ref().map(|b| b.chars().take(100).collect::<String>()),
            }))?;

            sqlx::query!(
                "UPDATE mailboxes SET last_uid_next = $1 WHERE id = $2",
                uid as i64,
                mailbox.id
            )
            .execute(pool)
            .await?;
        }
    }

    session.logout().await?;
    Ok(())
}
