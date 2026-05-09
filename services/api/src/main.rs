mod auth;
mod db;
mod mail;
mod realtime;

use axum::{routing::{get, post}, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let pool = db::connect_db().await;

    let app = Router::new()
        .route("/", get(|| async { "Webmail API is running" }))
        .route("/ws", get(realtime::ws_handler))
        .route("/auth/register", post(auth::register_user))
        .route("/auth/login", post(auth::login_user))
        .route("/mailboxes", get(mail::list_mailboxes))
        .route("/mailboxes/:id/messages", get(mail::list_messages))
        .route("/mail/send", post(mail::send_email))
        .with_state(pool);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
