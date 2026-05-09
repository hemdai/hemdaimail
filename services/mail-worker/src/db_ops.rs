use sqlx::PgPool;
use uuid::Uuid;
use std::error::Error;
use crate::imap::processor::ProcessedMessage;

pub async fn save_message(
    pool: &PgPool,
    user_id: Uuid,
    mailbox_id: Uuid,
    remote_uid: u32,
    msg: ProcessedMessage,
    s3_key: String,
) -> Result<Uuid, Box<dyn Error>> {
    // 1. Thread Reconstruction
    let thread_id = find_or_create_thread(pool, &msg).await?;

    // 2. Save Message
    let message = sqlx::query!(
        r#"
        INSERT INTO messages (
            user_id, mailbox_id, thread_id, remote_uid, message_id,
            sender, recipients, subject, body_text, body_html,
            in_reply_to, references_list, s3_key
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
        ON CONFLICT (mailbox_id, remote_uid) DO UPDATE SET message_id = EXCLUDED.message_id
        RETURNING id
        "#,
        user_id,
        mailbox_id,
        thread_id,
        remote_uid as i64,
        msg.message_id,
        msg.from.unwrap_or_default(),
        &msg.to,
        msg.subject,
        msg.body_text,
        msg.body_html,
        msg.in_reply_to,
        &msg.references,
        s3_key
    )
    .fetch_one(pool)
    .await?;

    Ok(message.id)
}

pub async fn save_attachment(
    pool: &PgPool,
    message_id: Uuid,
    filename: &str,
    content_type: &str,
    size: usize,
    s3_key: &str,
) -> Result<(), Box<dyn Error>> {
    sqlx::query!(
        r#"
        INSERT INTO attachments (message_id, filename, content_type, size, s3_key)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        message_id,
        filename,
        content_type,
        size as i64,
        s3_key
    )
    .execute(pool)
    .await?;

    Ok(())
}

async fn find_or_create_thread(
    pool: &PgPool,
    msg: &ProcessedMessage,
) -> Result<Uuid, Box<dyn Error>> {
    // Try to find thread by In-Reply-To
    if let Some(ref in_reply_to) = msg.in_reply_to {
        let thread = sqlx::query!(
            "SELECT thread_id FROM messages WHERE message_id = $1 LIMIT 1",
            in_reply_to
        )
        .fetch_optional(pool)
        .await?;

        if let Some(row) = thread {
            if let Some(tid) = row.thread_id {
                return Ok(tid);
            }
        }
    }

    // Try to find thread by References
    for reference in &msg.references {
        let thread = sqlx::query!(
            "SELECT thread_id FROM messages WHERE message_id = $1 LIMIT 1",
            reference
        )
        .fetch_optional(pool)
        .await?;

        if let Some(row) = thread {
            if let Some(tid) = row.thread_id {
                return Ok(tid);
            }
        }
    }

    // Create new thread
    let thread = sqlx::query!(
        "INSERT INTO threads (subject) VALUES ($1) RETURNING id",
        msg.subject
    )
    .fetch_one(pool)
    .await?;

    Ok(thread.id)
}
