mod auth;
mod db;
mod mail;
mod realtime;
mod observability;

use axum::{routing::{get, post}, Router};
use std::net::SocketAddr;
use metrics_exporter_prometheus::PrometheusBuilder;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    observability::init_observability("api-service");

    let recorder_handle = PrometheusBuilder::new()
        .install_recorder()
        .expect("failed to install recorder");

    let pool = db::connect_db().await;

    let app = Router::new()
        .route("/", get(|| async { "Webmail API is running" }))
        .route("/health", get(health_handler))
        .route("/metrics", get(move || async move { recorder_handle.render() }))
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

async fn health_handler(
    axum::extract::State(pool): axum::extract::State<sqlx::PgPool>,
) -> impl axum::response::IntoResponse {
    match sqlx::query("SELECT 1").execute(&pool).await {
        Ok(_) => (axum::http::StatusCode::OK, "OK"),
        Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "DB Connection Failed"),
    }
}
