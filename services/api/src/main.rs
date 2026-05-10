use api::{app, db};
use std::net::SocketAddr;
use metrics_exporter_prometheus::PrometheusBuilder;
use axum::routing::get;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    api::observability::init_observability("api-service");

    let recorder_handle = PrometheusBuilder::new()
        .install_recorder()
        .expect("failed to install recorder");

    let pool = db::connect_db().await;

    let router = app(pool).await;
    let final_router = router.route("/metrics", get(move || async move { recorder_handle.render() }));

    let addr = SocketAddr::from(([0, 0, 0, 0], 4000));
    tracing::info!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, final_router).await.unwrap();
}
