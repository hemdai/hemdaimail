pub mod models;

use axum::{
    extract::{State, Path, Query},
    Json,
    response::IntoResponse,
    http::StatusCode,
};
use sqlx::PgPool;
use models::{Mailbox, Message, SendEmailRequest};
use crate::auth::models::Claims;
use serde::Deserialize;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message as LettreMessage, SmtpTransport, Transport};
use std::env;

#[derive(Debug, Deserialize)]
pub struct Pagination {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub async fn list_mailboxes(
    State(pool): State<PgPool>,
    claims: Claims,
) -> impl IntoResponse {
    let mailboxes = sqlx::query_as!(
        Mailbox,
        r#"SELECT id, user_id, name, type as "type: _", created_at FROM mailboxes WHERE user_id = $1"#,
        claims.sub
    )
    .fetch_all(&pool)
    .await;

    match mailboxes {
        Ok(m) => Json(m).into_response(),
        Err(e) => {
            tracing::error!("Failed to list mailboxes: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn list_messages(
    State(pool): State<PgPool>,
    claims: Claims,
    Path(mailbox_id): Path<uuid::Uuid>,
    Query(pagination): Query<Pagination>,
) -> impl IntoResponse {
    let limit = pagination.limit.unwrap_or(50);
    let offset = pagination.offset.unwrap_or(0);

    let messages = sqlx::query_as!(
        Message,
        "SELECT id, user_id, mailbox_id, thread_id, sender, recipients, subject, snippet, is_read, is_starred, is_archived, created_at FROM messages WHERE user_id = $1 AND mailbox_id = $2 ORDER BY created_at DESC LIMIT $3 OFFSET $4",
        claims.sub,
        mailbox_id,
        limit,
        offset
    )
    .fetch_all(&pool)
    .await;

    match messages {
        Ok(m) => Json(m).into_response(),
        Err(e) => {
            tracing::error!("Failed to list messages: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn send_email(
    State(pool): State<PgPool>,
    claims: Claims,
    Json(payload): Json<SendEmailRequest>,
) -> impl IntoResponse {
    // 1. Prepare Email
    let email = LettreMessage::builder()
        .from(claims.email.parse().unwrap())
        .to(payload.to[0].parse().unwrap()) // Simplification: taking first recipient
        .subject(&payload.subject)
        .body(payload.body.clone())
        .expect("Failed to build email");

    // 2. Setup SMTP Transport
    let smtp_host = env::var("SMTP_HOST").expect("SMTP_HOST must be set");
    let smtp_user = env::var("SMTP_USER").expect("SMTP_USER must be set");
    let smtp_pass = env::var("SMTP_PASS").expect("SMTP_PASS must be set");

    let creds = Credentials::new(smtp_user, smtp_pass);
    let mailer = SmtpTransport::relay(&smtp_host)
        .unwrap()
        .credentials(creds)
        .build();

    // 3. Send Email
    match mailer.send(&email) {
        Ok(_) => {
            // 4. Save to Sent Mailbox
            // Find or create 'Sent' mailbox
            let mailbox = sqlx::query_as!(
                Mailbox,
                r#"SELECT id, user_id, name, type as "type: _", created_at FROM mailboxes WHERE user_id = $1 AND type = 'sent'"#,
                claims.sub
            )
            .fetch_optional(&pool)
            .await;

            let mailbox_id = match mailbox {
                Ok(Some(m)) => m.id,
                _ => {
                    // Create it if it doesn't exist
                    let m = sqlx::query!(
                        "INSERT INTO mailboxes (user_id, name, type) VALUES ($1, 'Sent', 'sent') RETURNING id",
                        claims.sub
                    )
                    .fetch_one(&pool)
                    .await
                    .expect("Failed to create Sent mailbox");
                    m.id
                }
            };

            // Save message record
            let recipients: Vec<String> = payload.to.clone();
            let result = sqlx::query!(
                "INSERT INTO messages (user_id, mailbox_id, sender, recipients, subject, body_text, is_read) VALUES ($1, $2, $3, $4, $5, $6, TRUE)",
                claims.sub,
                mailbox_id,
                claims.email,
                &recipients,
                payload.subject,
                payload.body
            )
            .execute(&pool)
            .await;

            match result {
                Ok(_) => StatusCode::OK.into_response(),
                Err(e) => {
                    tracing::error!("Failed to save sent message: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            }
        }
        Err(e) => {
            tracing::error!("Could not send email: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
