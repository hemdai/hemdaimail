mod auth;
mod db;
mod mail;
mod realtime;
mod observability;

use axum::{routing::{get, post}, Router, http::header::HeaderName};
use std::net::SocketAddr;
use metrics_exporter_prometheus::PrometheusBuilder;
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};
use std::sync::Arc;
use tower_http::{
    request_id::{MakeRequestId, RequestId, PropagateRequestIdLayer, SetRequestIdLayer},
    trace::TraceLayer,
};
use uuid::Uuid;

#[derive(Clone, Copy)]
struct MyMakeRequestId;

impl MakeRequestId for MyMakeRequestId {
    fn make_request_id<B>(&mut self, _request: &axum::http::Request<B>) -> Option<RequestId> {
        let request_id = Uuid::new_v4().to_string().parse().unwrap();
        Some(RequestId::new(request_id))
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    observability::init_observability("api-service");

    let x_request_id = HeaderName::from_static("x-request-id");

    let recorder_handle = PrometheusBuilder::new()
        .install_recorder()
        .expect("failed to install recorder");

    // Rate Limiting Config
    let general_governor_config = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(10)
            .burst_size(20)
            .finish()
            .unwrap(),
    );

    let auth_governor_config = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(1)
            .burst_size(5)
            .finish()
            .unwrap(),
    );

    let pool = db::connect_db().await;

    let auth_routes = Router::new()
        .route("/register", post(auth::register_user))
        .route("/login", post(auth::login_user))
        .route("/refresh", post(auth::refresh_token))
        .layer(GovernorLayer {
            config: auth_governor_config,
        });

    let mail_routes = Router::new()
        .route("/mailboxes", get(mail::list_mailboxes))
        .route("/mailboxes/:id/messages", get(mail::list_messages))
        .route("/mail/send", post(mail::send_email));

    let app = Router::new()
        .route("/", get(|| async { "Webmail API is running" }))
        .route("/health", get(health_handler))
        .route("/metrics", get(move || async move { recorder_handle.render() }))
        .route("/ws", get(realtime::ws_handler))
        .nest("/auth", auth_routes)
        .nest("/", mail_routes)
        .layer(GovernorLayer {
            config: general_governor_config,
        })
        .layer(TraceLayer::new_for_http())
        .layer(PropagateRequestIdLayer::new(x_request_id.clone()))
        .layer(SetRequestIdLayer::new(x_request_id, MyMakeRequestId))
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
