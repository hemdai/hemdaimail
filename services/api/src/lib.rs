pub mod auth;
pub mod db;
pub mod mail;
pub mod realtime;
pub mod observability;

use axum::{routing::{get, post}, Router, http::header::HeaderName, response::IntoResponse};
use std::sync::Arc;
use tower_http::{
    request_id::{MakeRequestId, RequestId, PropagateRequestIdLayer, SetRequestIdLayer},
    trace::TraceLayer,
};
use uuid::Uuid;
use sqlx::PgPool;

#[derive(Clone, Copy)]
struct MyMakeRequestId;

impl MakeRequestId for MyMakeRequestId {
    fn make_request_id<B>(&mut self, _request: &axum::http::Request<B>) -> Option<RequestId> {
        let request_id = Uuid::new_v4().to_string().parse().unwrap();
        Some(RequestId::new(request_id))
    }
}

pub async fn app(pool: PgPool) -> Router {
    let x_request_id = HeaderName::from_static("x-request-id");

    let auth_routes = Router::new()
        .route("/register", post(auth::register_user))
        .route("/login", post(auth::login_user))
        .route("/refresh", post(auth::refresh_token));

    let mail_routes = Router::new()
        .route("/mailboxes", get(mail::list_mailboxes))
        .route("/mailboxes/:id/messages", get(mail::list_messages))
        .route("/mail/send", post(mail::send_email));

    Router::new()
        .route("/", get(|| async { "Webmail API is running" }))
        .route("/health", get(health_handler))
        .route("/ws", get(realtime::ws_handler))
        .nest("/auth", auth_routes)
        .nest("/", mail_routes)
        .layer(TraceLayer::new_for_http())
        .layer(PropagateRequestIdLayer::new(x_request_id.clone()))
        .layer(SetRequestIdLayer::new(x_request_id, MyMakeRequestId))
        .with_state(pool)
}

async fn health_handler(
    axum::extract::State(pool): axum::extract::State<PgPool>,
) -> impl IntoResponse {
    match sqlx::query("SELECT 1").execute(&pool).await {
        Ok(_) => (axum::http::StatusCode::OK, "OK"),
        Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "DB Connection Failed"),
    }
}
