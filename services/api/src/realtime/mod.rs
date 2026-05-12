use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
};
use futures_util::StreamExt;
use crate::auth::models::Claims;
use sqlx::PgPool;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    claims: Claims,
    State(pool): State<PgPool>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, claims, pool))
}

async fn handle_socket(mut socket: WebSocket, claims: Claims, _pool: PgPool) {
    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");
    let client = redis::Client::open(redis_url).expect("Invalid Redis URL");
    let conn = client.get_async_connection().await.expect("Failed to connect to Redis");
    let mut pubsub_conn = conn.into_pubsub();

    let channel = format!("user_events:{}", claims.sub);
    pubsub_conn.subscribe(&channel).await.expect("Failed to subscribe");

    let mut pubsub_stream = pubsub_conn.on_message();

    tracing::info!("User {} connected to WebSocket", claims.sub);

    loop {
        tokio::select! {
            // Message from Redis Pub/Sub
            Some(msg) = pubsub_stream.next() => {
                let payload: String = msg.get_payload().expect("Failed to get Redis payload");
                if socket.send(Message::Text(payload)).await.is_err() {
                    break;
                }
            }
            // Message from WebSocket Client
            Some(result) = socket.next() => {
                match result {
                    Ok(msg) => {
                        if let Message::Close(_) = msg {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        }
    }

    tracing::info!("User {} disconnected from WebSocket", claims.sub);
}
