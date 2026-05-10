use sqlx::PgPool;
use uuid::Uuid;
use serde_json::Value;

pub async fn log_event(
    pool: &PgPool,
    user_id: Option<Uuid>,
    action: &str,
    resource_type: &str,
    resource_id: Option<&str>,
    payload: Option<Value>,
    ip_address: Option<&str>,
    user_agent: Option<&str>,
) {
    let result = sqlx::query!(
        r#"
        INSERT INTO audit_logs (user_id, action, resource_type, resource_id, payload, ip_address, user_agent)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
        user_id,
        action,
        resource_type,
        resource_id,
        payload,
        ip_address,
        user_agent
    )
    .execute(pool)
    .await;

    if let Err(e) = result {
        tracing::error!("Failed to log audit event: {}", e);
    }
}
